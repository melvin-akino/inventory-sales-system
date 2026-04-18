use crate::{
    auth_guard::{extract_token, validate_session},
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn login(
    State(db): State<Db>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request' field"))?;
    let username = req["username"].as_str().ok_or_else(|| bad_req("Missing username"))?;
    let password = req["password"].as_str().ok_or_else(|| bad_req("Missing password"))?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    let result = db.query_row(
        "SELECT u.id, u.username, u.password_hash, u.full_name, u.email, u.role, u.is_active, u.industry_id,
                i.code as industry_code, i.name as industry_name, u.created_at, u.updated_at
         FROM users u
         LEFT JOIN industries i ON u.industry_id = i.id
         WHERE u.username = ? AND u.is_active = 1",
        params![username],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, bool>(6)?,
                row.get::<_, Option<i64>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
            ))
        },
    );

    match result {
        Ok((id, uname, password_hash, full_name, email, role, is_active, industry_id, industry_code, industry_name, created_at, updated_at)) => {
            if !bcrypt::verify(password, &password_hash).unwrap_or(false) {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid username or password".to_string(),
                ));
            }

            let token = Uuid::new_v4().to_string();
            let expires_at = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(8))
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

            db.execute("DELETE FROM sessions WHERE user_id = ?", params![id]).ok();
            db.execute(
                "INSERT INTO sessions (token, user_id, expires_at) VALUES (?, ?, ?)",
                params![token, id, expires_at],
            )
            .map_err(|e| server_err(e.to_string()))?;

            let mut user_json = json!({
                "id": id,
                "username": uname,
                "full_name": full_name,
                "email": email,
                "role": role,
                "is_active": is_active,
                "created_at": created_at,
                "updated_at": updated_at
            });

            // Add industry information if available
            if let (Some(ind_id), Some(ind_code), Some(ind_name)) = (industry_id, industry_code, industry_name) {
                user_json["industry"] = json!({
                    "id": ind_id,
                    "code": ind_code,
                    "name": ind_name
                });
            }

            Ok(Json(json!({
                "token": token,
                "user": user_json
            })))
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        )),
    }
}

pub async fn logout(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Accept token from Authorization header or body
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or_else(|| body["token"].as_str().map(|s| s.to_string()))
        .ok_or_else(|| bad_req("Missing token"))?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    db.execute("DELETE FROM sessions WHERE token = ?", params![token])
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!(null)))
}

pub async fn get_current_user(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    let user = db.query_row(
        "SELECT u.id, u.username, u.full_name, u.email, u.role, u.is_active, u.industry_id,
                i.code as industry_code, i.name as industry_name, u.created_at, u.updated_at
         FROM sessions s
         JOIN users u ON s.user_id = u.id
         LEFT JOIN industries i ON u.industry_id = i.id
         WHERE s.token = ? AND s.expires_at > datetime('now') AND u.is_active = 1",
        params![token],
        |row| {
            let mut user_obj = json!({
                "id":         row.get::<_, i64>(0)?,
                "username":   row.get::<_, String>(1)?,
                "full_name":  row.get::<_, String>(2)?,
                "email":      row.get::<_, Option<String>>(3)?,
                "role":       row.get::<_, String>(4)?,
                "is_active":  row.get::<_, bool>(5)?,
                "created_at": row.get::<_, String>(9)?,
                "updated_at": row.get::<_, String>(10)?
            });

            // Add industry if available
            if let Ok(Some(ind_id)) = row.get::<_, Option<i64>>(6) {
                if let (Ok(Some(ind_code)), Ok(Some(ind_name))) = (
                    row.get::<_, Option<String>>(7),
                    row.get::<_, Option<String>>(8)
                ) {
                    user_obj["industry"] = json!({
                        "id": ind_id,
                        "code": ind_code,
                        "name": ind_name
                    });
                }
            }

            Ok(user_obj)
        },
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Session expired or invalid".to_string()))?;

    Ok(Json(user))
}

// Helper used by other routes to get the token (header or body fallback)
pub fn get_token(headers: &HeaderMap, body: &Value) -> Result<String, (StatusCode, String)> {
    extract_token(headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })
}

// Get industry ID from user's session
pub fn get_user_industry_id(db: &rusqlite::Connection, token: &str) -> Result<Option<i64>, (StatusCode, String)> {
    db.query_row(
        "SELECT u.industry_id FROM sessions s
         JOIN users u ON s.user_id = u.id
         WHERE s.token = ? AND s.expires_at > datetime('now') AND u.is_active = 1",
        params![token],
        |row| row.get::<_, Option<i64>>(0),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Session expired or invalid".to_string()))
}
