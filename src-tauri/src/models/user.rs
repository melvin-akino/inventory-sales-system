use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub email: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub email: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub full_name: String,
    pub role: String,
}
