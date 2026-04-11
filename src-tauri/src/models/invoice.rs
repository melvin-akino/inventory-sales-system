use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invoice {
    pub id: i64,
    pub sale_id: i64,
    pub sale_number: String,
    pub invoice_number: String,
    pub invoice_date: String,
    pub due_date: Option<String>,
    pub customer_name: Option<String>,
    pub customer_tin: Option<String>,
    pub customer_address: Option<String>,
    pub status: String,
    pub total_amount: f64,
    pub vat_amount: f64,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceFilter {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}
