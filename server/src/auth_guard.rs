use axum::http::{HeaderMap, StatusCode};
use rusqlite::params;

pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub full_name: String,
    pub role: String,
}

/// Extract Bearer token from Authorization header.
pub fn extract_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header".to_string(),
            )
        })
}

/// Validate session token against the database.
pub fn validate_session(
    db: &rusqlite::Connection,
    token: &str,
) -> Result<SessionUser, (StatusCode, String)> {
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
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Unauthorized: session invalid or expired".to_string(),
        )
    })
}

/// Assert that the session user has one of the allowed roles.
pub fn require_role(
    session: &SessionUser,
    allowed_roles: &[&str],
) -> Result<(), (StatusCode, String)> {
    if allowed_roles.contains(&session.role.as_str()) {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!(
                "Access denied: role '{}' is not permitted for this action",
                session.role
            ),
        ))
    }
}
