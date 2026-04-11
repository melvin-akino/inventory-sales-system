use crate::commands::auth::{require_role, validate_session};
use crate::AppState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: Option<String>,
}

#[tauri::command]
pub async fn get_settings(
    token: String,
    state: State<'_, AppState>,
) -> Result<HashMap<String, String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| e.to_string())?;

    let map: HashMap<String, String> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(map)
}

#[tauri::command]
pub async fn update_settings(
    token: String,
    settings: HashMap<String, String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    for (key, value) in settings {
        db.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![key, value],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
