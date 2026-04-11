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

fn row_to_supplier(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    Ok(json!({
        "id":             row.get::<_, i64>(0)?,
        "name":           row.get::<_, String>(1)?,
        "contact_person": row.get::<_, Option<String>>(2)?,
        "phone":          row.get::<_, Option<String>>(3)?,
        "email":          row.get::<_, Option<String>>(4)?,
        "address":        row.get::<_, Option<String>>(5)?,
        "tin_number":     row.get::<_, Option<String>>(6)?,
        "is_active":      row.get::<_, bool>(7)?,
        "created_at":     row.get::<_, String>(8)?
    }))
}

pub async fn get_suppliers(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let sql = match body["search"].as_str() {
        Some(s) => format!(
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

    let mut stmt = db.prepare(&sql).map_err(|e| server_err(e.to_string()))?;
    let suppliers: Vec<Value> = stmt
        .query_map([], row_to_supplier)
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(suppliers)))
}

pub async fn create_supplier(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;

    let contact_person = req["contact_person"].as_str().or_else(|| req["contactPerson"].as_str());
    let phone = req["phone"].as_str();
    let email = req["email"].as_str();
    let address = req["address"].as_str();
    let tin_number = req["tin_number"].as_str().or_else(|| req["tinNumber"].as_str());

    db.execute(
        "INSERT INTO suppliers (name, contact_person, phone, email, address, tin_number) VALUES (?, ?, ?, ?, ?, ?)",
        params![name.trim(), contact_person, phone, email, address, tin_number],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let id = db.last_insert_rowid();
    let supplier = db
        .query_row(
            "SELECT id, name, contact_person, phone, email, address, tin_number, is_active, created_at FROM suppliers WHERE id = ?",
            params![id],
            row_to_supplier,
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(supplier))
}

pub async fn update_supplier(
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
                    concat!("UPDATE suppliers SET ", $col, " = ? WHERE id = ?"),
                    params![val, id],
                )
                .map_err(|e| server_err(e.to_string()))?;
            }
        };
    }

    upd!("name",           "name",          "name");
    upd!("contact_person", "contactPerson", "contact_person");
    upd!("phone",          "phone",         "phone");
    upd!("email",          "email",         "email");
    upd!("address",        "address",       "address");
    upd!("tin_number",     "tinNumber",     "tin_number");

    if let Some(is_active) = req["is_active"].as_bool().or_else(|| req["isActive"].as_bool()) {
        db.execute("UPDATE suppliers SET is_active = ? WHERE id = ?", params![is_active, id])
            .map_err(|e| server_err(e.to_string()))?;
    }

    let supplier = db
        .query_row(
            "SELECT id, name, contact_person, phone, email, address, tin_number, is_active, created_at FROM suppliers WHERE id = ?",
            params![id],
            row_to_supplier,
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(supplier))
}

pub async fn delete_supplier(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    db.execute("UPDATE suppliers SET is_active = 0 WHERE id = ?", params![id])
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!(null)))
}
