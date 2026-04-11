use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub cost_price: f64,
    pub selling_price: f64,
    pub quantity: i64,
    pub reorder_level: i64,
    pub is_vat_exempt: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub category_id: Option<i64>,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub cost_price: f64,
    pub selling_price: f64,
    pub quantity: i64,
    pub reorder_level: i64,
    pub is_vat_exempt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub category_id: Option<i64>,
    pub sku: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub cost_price: Option<f64>,
    pub selling_price: Option<f64>,
    pub reorder_level: Option<i64>,
    pub is_vat_exempt: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockAdjustmentRequest {
    pub product_id: i64,
    pub adjustment_type: String,
    pub quantity: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockAdjustment {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub adjustment_type: String,
    pub quantity_before: i64,
    pub quantity_change: i64,
    pub quantity_after: i64,
    pub reason: Option<String>,
    pub user_id: i64,
    pub adjusted_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilter {
    pub search: Option<String>,
    pub category_id: Option<i64>,
    pub low_stock_only: Option<bool>,
    pub active_only: Option<bool>,
}
