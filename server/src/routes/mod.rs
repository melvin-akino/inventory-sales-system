pub mod auth;
pub mod customers;
pub mod inventory;
pub mod invoices;
pub mod reports;
pub mod sales;
pub mod settings;
pub mod suppliers;
pub mod users;

use axum::http::StatusCode;

pub fn bad_req(msg: impl ToString) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, msg.to_string())
}

pub fn server_err(msg: impl ToString) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string())
}
