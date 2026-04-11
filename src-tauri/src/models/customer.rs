use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tin_number: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tin_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tin_number: Option<String>,
    pub is_active: Option<bool>,
}
