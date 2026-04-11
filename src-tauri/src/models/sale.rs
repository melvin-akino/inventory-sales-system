use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaleItem {
    pub id: Option<i64>,
    pub sale_id: Option<i64>,
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub discount_percent: f64,
    pub vat_amount: f64,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sale {
    pub id: i64,
    pub sale_number: String,
    pub customer_id: Option<i64>,
    pub customer_name: Option<String>,
    pub user_id: i64,
    pub cashier_name: String,
    pub sale_date: String,
    pub subtotal: f64,
    pub discount_amount: f64,
    pub vat_amount: f64,
    pub total_amount: f64,
    pub amount_paid: f64,
    pub change_amount: f64,
    pub payment_method: String,
    pub status: String,
    pub notes: Option<String>,
    pub items: Vec<SaleItem>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSaleRequest {
    pub customer_id: Option<i64>,
    pub items: Vec<SaleItemRequest>,
    pub discount_amount: f64,
    pub amount_paid: f64,
    pub payment_method: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleItemRequest {
    pub product_id: i64,
    pub quantity: i64,
    pub unit_price: f64,
    pub discount_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleFilter {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub payment_method: Option<String>,
    pub status: Option<String>,
    pub customer_id: Option<i64>,
    pub search: Option<String>,
}
