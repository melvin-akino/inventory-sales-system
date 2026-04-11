use crate::models::user::{AuthResponse, LoginRequest, SessionUser, User};
use crate::AppState;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

fn hash_password(password: &str) -> Result<String, String> {
    bcrypt::hash(password, 12).map_err(|e| e.to_string())
}

fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

#[tauri::command]
pub async fn login(
    request: LoginRequest,
    state: State<'_, AppState>,
) -> Result<AuthResponse, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let result = db.query_row(
        "SELECT id, username, password_hash, full_name, email, role, is_active, created_at, updated_at
         FROM users WHERE username = ? AND is_active = 1",
        params![request.username],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, bool>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
            ))
        },
    );

    match result {
        Ok((id, username, password_hash, full_name, email, role, is_active, created_at, updated_at)) => {
            if !verify_password(&request.password, &password_hash) {
                return Err("Invalid username or password".to_string());
            }

            let token = Uuid::new_v4().to_string();
            let expires_at = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(8))
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

            // Clean up old sessions for this user
            db.execute("DELETE FROM sessions WHERE user_id = ?", params![id])
                .ok();

            db.execute(
                "INSERT INTO sessions (token, user_id, expires_at) VALUES (?, ?, ?)",
                params![token, id, expires_at],
            )
            .map_err(|e| e.to_string())?;

            Ok(AuthResponse {
                token,
                user: User {
                    id,
                    username,
                    full_name,
                    email,
                    role,
                    is_active,
                    created_at,
                    updated_at,
                },
            })
        }
        Err(_) => Err("Invalid username or password".to_string()),
    }
}

#[tauri::command]
pub async fn logout(token: String, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.execute("DELETE FROM sessions WHERE token = ?", params![token])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_current_user(
    token: String,
    state: State<'_, AppState>,
) -> Result<User, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let result = db.query_row(
        "SELECT u.id, u.username, u.full_name, u.email, u.role, u.is_active, u.created_at, u.updated_at
         FROM sessions s
         JOIN users u ON s.user_id = u.id
         WHERE s.token = ? AND s.expires_at > datetime('now') AND u.is_active = 1",
        params![token],
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
    );

    result.map_err(|_| "Session expired or invalid".to_string())
}

pub fn validate_session(db: &rusqlite::Connection, token: &str) -> Result<SessionUser, String> {
    db.query_row(
        "SELECT u.id, u.username, u.full_name, u.role
         FROM sessions s
         JOIN users u ON s.user_id = u.id
         WHERE s.token = ? AND s.expires_at > datetime('now') AND u.is_active = 1",
        params![token],
        |row| {
            Ok(SessionUser {
                user_id: row.get(0)?,
                username: row.get(1)?,
                full_name: row.get(2)?,
                role: row.get(3)?,
            })
        },
    )
    .map_err(|_| "Unauthorized: session invalid or expired".to_string())
}

pub fn require_role(session: &SessionUser, allowed_roles: &[&str]) -> Result<(), String> {
    if allowed_roles.contains(&session.role.as_str()) {
        Ok(())
    } else {
        Err(format!(
            "Access denied: role '{}' is not permitted for this action",
            session.role
        ))
    }
}
