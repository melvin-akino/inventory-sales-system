use crate::{
    auth_guard::{require_role, validate_session},
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};

use super::auth::get_token;

fn row_to_customer(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    Ok(json!({
        "id":         row.get::<_, i64>(0)?,
        "name":       row.get::<_, String>(1)?,
        "phone":      row.get::<_, Option<String>>(2)?,
        "email":      row.get::<_, Option<String>>(3)?,
        "address":    row.get::<_, Option<String>>(4)?,
        "tin_number": row.get::<_, Option<String>>(5)?,
        "is_active":  row.get::<_, bool>(6)?,
        "created_at": row.get::<_, String>(7)?
    }))
}

pub async fn get_customers(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let sql = match body["search"].as_str() {
        Some(s) => format!(
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

    let mut stmt = db.prepare(&sql).map_err(|e| server_err(e.to_string()))?;
    let customers: Vec<Value> = stmt
        .query_map([], row_to_customer)
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(customers)))
}

pub async fn create_customer(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager", "cashier"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;

    if name.trim().is_empty() {
        return Err(bad_req("Customer name cannot be empty"));
    }

    let phone = req["phone"].as_str();
    let email = req["email"].as_str();
    let address = req["address"].as_str();
    let tin_number = req["tin_number"].as_str().or_else(|| req["tinNumber"].as_str());

    db.execute(
        "INSERT INTO customers (name, phone, email, address, tin_number) VALUES (?, ?, ?, ?, ?)",
        params![name.trim(), phone, email, address, tin_number],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let id = db.last_insert_rowid();
    let customer = db
        .query_row(
            "SELECT id, name, phone, email, address, tin_number, is_active, created_at FROM customers WHERE id = ?",
            params![id],
            row_to_customer,
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(customer))
}

pub async fn update_customer(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    macro_rules! upd {
        ($key:expr, $alt:expr, $col:literal) => {
            if let Some(val) = req[$key].as_str().or_else(|| req[$alt].as_str()) {
                db.execute(
                    concat!("UPDATE customers SET ", $col, " = ? WHERE id = ?"),
                    params![val, id],
                )
                .map_err(|e| server_err(e.to_string()))?;
            }
        };
    }

    upd!("name",       "name",       "name");
    upd!("phone",      "phone",      "phone");
    upd!("email",      "email",      "email");
    upd!("address",    "address",    "address");
    upd!("tin_number", "tinNumber",  "tin_number");

    if let Some(is_active) = req["is_active"].as_bool().or_else(|| req["isActive"].as_bool()) {
        db.execute("UPDATE customers SET is_active = ? WHERE id = ?", params![is_active, id])
            .map_err(|e| server_err(e.to_string()))?;
    }

    let customer = db
        .query_row(
            "SELECT id, name, phone, email, address, tin_number, is_active, created_at FROM customers WHERE id = ?",
            params![id],
            row_to_customer,
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(customer))
}

pub async fn delete_customer(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    db.execute("UPDATE customers SET is_active = 0 WHERE id = ?", params![id])
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!(null)))
}
