use crate::commands::auth::{require_role, validate_session};
use crate::models::supplier::{CreateSupplierRequest, Supplier, UpdateSupplierRequest};
use crate::AppState;
use rusqlite::params;
use tauri::State;

fn row_to_supplier(row: &rusqlite::Row) -> rusqlite::Result<Supplier> {
    Ok(Supplier {
        id: row.get(0)?,
        name: row.get(1)?,
        contact_person: row.get(2)?,
        phone: row.get(3)?,
        email: row.get(4)?,
        address: row.get(5)?,
        tin_number: row.get(6)?,
        is_active: row.get(7)?,
        created_at: row.get(8)?,
    })
}

#[tauri::command]
pub async fn get_suppliers(
    token: String,
    search: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Supplier>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let sql = match search {
        Some(ref s) => format!(
            "SELECT id, name, contact_person, phone, email, address, tin_number, is_active, created_at
             FROM suppliers WHERE is_active = 1
             AND (name LIKE '%{0}%' OR contact_person LIKE '%{0}%')
             ORDER BY name",
            s.replace('\'', "")
        ),
        None => "SELECT id, name, contact_person, phone, email, address, tin_number, is_active, created_at
                 FROM suppliers WHERE is_active = 1 ORDER BY name"
            .to_string(),
    };

    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;
    let suppliers = stmt
        .query_map([], row_to_supplier)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(suppliers)
}

#[tauri::command]
pub async fn create_supplier(
    token: String,
    request: CreateSupplierRequest,
    state: State<'_, AppState>,
) -> Result<Supplier, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    db.execute(
        "INSERT INTO suppliers (name, contact_person, phone, email, address, tin_number) VALUES (?, ?, ?, ?, ?, ?)",
        params![
            request.name.trim(),
            request.contact_person,
            request.phone,
            request.email,
            request.address,
            request.tin_number
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, name, contact_person, phone, email, address, tin_number, is_active, created_at FROM suppliers WHERE id = ?",
        params![id],
        row_to_supplier,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_supplier(
    token: String,
    id: i64,
    request: UpdateSupplierRequest,
    state: State<'_, AppState>,
) -> Result<Supplier, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    macro_rules! upd {
        ($field:expr, $col:literal) => {
            if let Some(val) = $field {
                db.execute(
                    concat!("UPDATE suppliers SET ", $col, " = ? WHERE id = ?"),
                    params![val, id],
                )
                .map_err(|e| e.to_string())?;
            }
        };
    }

    upd!(request.name.as_deref(), "name");
    upd!(request.contact_person.as_deref(), "contact_person");
    upd!(request.phone.as_deref(), "phone");
    upd!(request.email.as_deref(), "email");
    upd!(request.address.as_deref(), "address");
    upd!(request.tin_number.as_deref(), "tin_number");
    upd!(request.is_active, "is_active");

    db.query_row(
        "SELECT id, name, contact_person, phone, email, address, tin_number, is_active, created_at FROM suppliers WHERE id = ?",
        params![id],
        row_to_supplier,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_supplier(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    db.execute(
        "UPDATE suppliers SET is_active = 0 WHERE id = ?",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
