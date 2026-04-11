use crate::commands::auth::{require_role, validate_session};
use crate::models::customer::{CreateCustomerRequest, Customer, UpdateCustomerRequest};
use crate::AppState;
use rusqlite::params;
use tauri::State;

fn row_to_customer(row: &rusqlite::Row) -> rusqlite::Result<Customer> {
    Ok(Customer {
        id: row.get(0)?,
        name: row.get(1)?,
        phone: row.get(2)?,
        email: row.get(3)?,
        address: row.get(4)?,
        tin_number: row.get(5)?,
        is_active: row.get(6)?,
        created_at: row.get(7)?,
    })
}

#[tauri::command]
pub async fn get_customers(
    token: String,
    search: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Customer>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    let sql = match search {
        Some(ref s) => format!(
            "SELECT id, name, phone, email, address, tin_number, is_active, created_at
             FROM customers WHERE is_active = 1
             AND (name LIKE '%{0}%' OR phone LIKE '%{0}%' OR tin_number LIKE '%{0}%')
             ORDER BY name",
            s.replace('\'', "")
        ),
        None => "SELECT id, name, phone, email, address, tin_number, is_active, created_at
                 FROM customers WHERE is_active = 1 ORDER BY name"
            .to_string(),
    };

    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;
    let customers = stmt
        .query_map([], row_to_customer)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(customers)
}

#[tauri::command]
pub async fn create_customer(
    token: String,
    request: CreateCustomerRequest,
    state: State<'_, AppState>,
) -> Result<Customer, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager", "cashier"])?;

    if request.name.trim().is_empty() {
        return Err("Customer name cannot be empty".to_string());
    }

    db.execute(
        "INSERT INTO customers (name, phone, email, address, tin_number) VALUES (?, ?, ?, ?, ?)",
        params![
            request.name.trim(),
            request.phone,
            request.email,
            request.address,
            request.tin_number
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, name, phone, email, address, tin_number, is_active, created_at FROM customers WHERE id = ?",
        params![id],
        row_to_customer,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_customer(
    token: String,
    id: i64,
    request: UpdateCustomerRequest,
    state: State<'_, AppState>,
) -> Result<Customer, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    macro_rules! upd {
        ($field:expr, $col:literal) => {
            if let Some(val) = $field {
                db.execute(
                    concat!("UPDATE customers SET ", $col, " = ? WHERE id = ?"),
                    params![val, id],
                )
                .map_err(|e| e.to_string())?;
            }
        };
    }

    upd!(request.name.as_deref(), "name");
    upd!(request.phone.as_deref(), "phone");
    upd!(request.email.as_deref(), "email");
    upd!(request.address.as_deref(), "address");
    upd!(request.tin_number.as_deref(), "tin_number");
    upd!(request.is_active, "is_active");

    db.query_row(
        "SELECT id, name, phone, email, address, tin_number, is_active, created_at FROM customers WHERE id = ?",
        params![id],
        row_to_customer,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_customer(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    db.execute(
        "UPDATE customers SET is_active = 0 WHERE id = ?",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
