use crate::{
    auth_guard::{require_role, validate_session},
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};
use std::collections::HashMap;

use super::auth::get_token;

pub async fn get_settings(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| server_err(e.to_string()))?;

    let map: HashMap<String, String> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            ))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(map)))
}

pub async fn update_settings(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    let settings = body["settings"]
        .as_object()
        .ok_or_else(|| bad_req("Missing 'settings' object"))?;

    for (key, value) in settings {
        let v = value.as_str().unwrap_or("");
        db.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![key, v],
        )
        .map_err(|e| server_err(e.to_string()))?;
    }

    Ok(Json(json!(null)))
}
