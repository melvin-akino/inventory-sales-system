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

fn row_to_user(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    Ok(json!({
        "id":         row.get::<_, i64>(0)?,
        "username":   row.get::<_, String>(1)?,
        "full_name":  row.get::<_, String>(2)?,
        "email":      row.get::<_, Option<String>>(3)?,
        "role":       row.get::<_, String>(4)?,
        "is_active":  row.get::<_, bool>(5)?,
        "created_at": row.get::<_, String>(6)?,
        "updated_at": row.get::<_, String>(7)?
    }))
}

pub async fn get_users(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let mut stmt = db
        .prepare(
            "SELECT id, username, full_name, email, role, is_active, created_at, updated_at
             FROM users ORDER BY full_name",
        )
        .map_err(|e| server_err(e.to_string()))?;

    let users: Vec<Value> = stmt
        .query_map([], row_to_user)
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(users)))
}

pub async fn create_user(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let username = req["username"].as_str().ok_or_else(|| bad_req("Missing username"))?;
    let password = req["password"].as_str().ok_or_else(|| bad_req("Missing password"))?;
    let full_name = req["full_name"].as_str()
        .or_else(|| req["fullName"].as_str())
        .ok_or_else(|| bad_req("Missing full_name"))?;
    let email = req["email"].as_str();
    let role = req["role"].as_str().ok_or_else(|| bad_req("Missing role"))?;

    if username.trim().is_empty() || password.len() < 6 {
        return Err(bad_req("Username cannot be empty and password must be at least 6 characters"));
    }

    let valid_roles = ["super_admin", "admin", "manager", "cashier", "viewer"];
    if !valid_roles.contains(&role) {
        return Err(bad_req(format!("Invalid role: {}", role)));
    }

    let hash = bcrypt::hash(password, 12).map_err(|e| server_err(e.to_string()))?;

    db.execute(
        "INSERT INTO users (username, password_hash, full_name, email, role) VALUES (?, ?, ?, ?, ?)",
        params![username.trim(), hash, full_name.trim(), email, role],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            bad_req("Username already exists")
        } else {
            server_err(e.to_string())
        }
    })?;

    let id = db.last_insert_rowid();
    let user = db
        .query_row(
            "SELECT id, username, full_name, email, role, is_active, created_at, updated_at FROM users WHERE id = ?",
            params![id],
            row_to_user,
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(user))
}

pub async fn update_user(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    if let Some(password) = req["password"].as_str() {
        if password.len() < 6 {
            return Err(bad_req("Password must be at least 6 characters"));
        }
        let hash = bcrypt::hash(password, 12).map_err(|e| server_err(e.to_string()))?;
        db.execute(
            "UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?",
            params![hash, id],
        )
        .map_err(|e| server_err(e.to_string()))?;
    }

    macro_rules! upd {
        ($key:expr, $alt:expr, $col:literal) => {
            if let Some(val) = req[$key].as_str().or_else(|| req[$alt].as_str()) {
                db.execute(
                    concat!("UPDATE users SET ", $col, " = ?, updated_at = datetime('now') WHERE id = ?"),
                    params![val, id],
                )
                .map_err(|e| server_err(e.to_string()))?;
            }
        };
    }

    upd!("full_name", "fullName", "full_name");
    upd!("email",     "email",    "email");
    upd!("role",      "role",     "role");

    if let Some(is_active) = req["is_active"].as_bool().or_else(|| req["isActive"].as_bool()) {
        db.execute(
            "UPDATE users SET is_active = ?, updated_at = datetime('now') WHERE id = ?",
            params![is_active, id],
        )
        .map_err(|e| server_err(e.to_string()))?;
    }

    let user = db
        .query_row(
            "SELECT id, username, full_name, email, role, is_active, created_at, updated_at FROM users WHERE id = ?",
            params![id],
            row_to_user,
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;

    if session.user_id == id {
        return Err(bad_req("Cannot delete your own account"));
    }

    db.execute("UPDATE users SET is_active = 0 WHERE id = ?", params![id])
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!(null)))
}

pub async fn change_password(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;

    // Support both camelCase (from JS) and snake_case
    let current_password = body["currentPassword"]
        .as_str()
        .or_else(|| body["current_password"].as_str())
        .ok_or_else(|| bad_req("Missing currentPassword"))?;

    let new_password = body["newPassword"]
        .as_str()
        .or_else(|| body["new_password"].as_str())
        .ok_or_else(|| bad_req("Missing newPassword"))?;

    if new_password.len() < 6 {
        return Err(bad_req("New password must be at least 6 characters"));
    }

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;

    let password_hash: String = db
        .query_row(
            "SELECT password_hash FROM users WHERE id = ?",
            params![session.user_id],
            |r| r.get(0),
        )
        .map_err(|e| server_err(e.to_string()))?;

    if !bcrypt::verify(current_password, &password_hash).unwrap_or(false) {
        return Err(bad_req("Current password is incorrect"));
    }

    let new_hash = bcrypt::hash(new_password, 12).map_err(|e| server_err(e.to_string()))?;
    db.execute(
        "UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?",
        params![new_hash, session.user_id],
    )
    .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!(null)))
}
