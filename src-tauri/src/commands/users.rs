use crate::commands::auth::{require_role, validate_session};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::AppState;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub async fn get_users(token: String, state: State<'_, AppState>) -> Result<Vec<User>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let mut stmt = db
        .prepare(
            "SELECT id, username, full_name, email, role, is_active, created_at, updated_at
             FROM users ORDER BY full_name",
        )
        .map_err(|e| e.to_string())?;

    let users = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                full_name: row.get(2)?,
                email: row.get(3)?,
                role: row.get(4)?,
                is_active: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(users)
}

#[tauri::command]
pub async fn create_user(
    token: String,
    request: CreateUserRequest,
    state: State<'_, AppState>,
) -> Result<User, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    if request.username.trim().is_empty() || request.password.len() < 6 {
        return Err("Username cannot be empty and password must be at least 6 characters".to_string());
    }

    let valid_roles = ["super_admin", "admin", "manager", "cashier", "viewer"];
    if !valid_roles.contains(&request.role.as_str()) {
        return Err(format!("Invalid role: {}", request.role));
    }

    let password_hash = bcrypt::hash(&request.password, 12).map_err(|e| e.to_string())?;

    db.execute(
        "INSERT INTO users (username, password_hash, full_name, email, role)
         VALUES (?, ?, ?, ?, ?)",
        params![
            request.username.trim(),
            password_hash,
            request.full_name.trim(),
            request.email,
            request.role
        ],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            "Username already exists".to_string()
        } else {
            e.to_string()
        }
    })?;

    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, username, full_name, email, role, is_active, created_at, updated_at
         FROM users WHERE id = ?",
        params![id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                full_name: row.get(2)?,
                email: row.get(3)?,
                role: row.get(4)?,
                is_active: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_user(
    token: String,
    id: i64,
    request: UpdateUserRequest,
    state: State<'_, AppState>,
) -> Result<User, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    if let Some(ref password) = request.password {
        if password.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
        }
        let hash = bcrypt::hash(password, 12).map_err(|e| e.to_string())?;
        db.execute(
            "UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?",
            params![hash, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(ref full_name) = request.full_name {
        db.execute(
            "UPDATE users SET full_name = ?, updated_at = datetime('now') WHERE id = ?",
            params![full_name, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(ref email) = request.email {
        db.execute(
            "UPDATE users SET email = ?, updated_at = datetime('now') WHERE id = ?",
            params![email, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(ref role) = request.role {
        db.execute(
            "UPDATE users SET role = ?, updated_at = datetime('now') WHERE id = ?",
            params![role, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(is_active) = request.is_active {
        db.execute(
            "UPDATE users SET is_active = ?, updated_at = datetime('now') WHERE id = ?",
            params![is_active, id],
        )
        .map_err(|e| e.to_string())?;
    }

    db.query_row(
        "SELECT id, username, full_name, email, role, is_active, created_at, updated_at
         FROM users WHERE id = ?",
        params![id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                full_name: row.get(2)?,
                email: row.get(3)?,
                role: row.get(4)?,
                is_active: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_user(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin"])?;

    if session.user_id == id {
        return Err("Cannot delete your own account".to_string());
    }

    db.execute("UPDATE users SET is_active = 0 WHERE id = ?", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn change_password(
    token: String,
    current_password: String,
    new_password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    if new_password.len() < 6 {
        return Err("New password must be at least 6 characters".to_string());
    }

    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;

    let password_hash: String = db
        .query_row(
            "SELECT password_hash FROM users WHERE id = ?",
            params![session.user_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    if !bcrypt::verify(&current_password, &password_hash).unwrap_or(false) {
        return Err("Current password is incorrect".to_string());
    }

    let new_hash = bcrypt::hash(&new_password, 12).map_err(|e| e.to_string())?;
    db.execute(
        "UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?",
        params![new_hash, session.user_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
