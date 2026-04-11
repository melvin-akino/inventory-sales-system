use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sales_today: f64,
    pub total_transactions_today: i64,
    pub total_products: i64,
    pub low_stock_count: i64,
    pub total_customers: i64,
    pub sales_this_month: f64,
    pub top_products: Vec<TopProduct>,
    pub recent_sales: Vec<RecentSale>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopProduct {
    pub product_name: String,
    pub quantity_sold: i64,
    pub total_revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentSale {
    pub sale_number: String,
    pub customer_name: Option<String>,
    pub total_amount: f64,
    pub payment_method: String,
    pub sale_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesReportItem {
    pub date: String,
    pub sale_number: String,
    pub customer_name: Option<String>,
    pub cashier_name: String,
    pub subtotal: f64,
    pub discount: f64,
    pub vat: f64,
    pub total: f64,
    pub payment_method: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesReportSummary {
    pub items: Vec<SalesReportItem>,
    pub total_sales: f64,
    pub total_vat: f64,
    pub total_discount: f64,
    pub grand_total: f64,
    pub transaction_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryReportItem {
    pub sku: String,
    pub product_name: String,
    pub category: Option<String>,
    pub unit: String,
    pub quantity: i64,
    pub cost_price: f64,
    pub selling_price: f64,
    pub inventory_value: f64,
    pub reorder_level: i64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryReportSummary {
    pub items: Vec<InventoryReportItem>,
    pub total_items: i64,
    pub total_inventory_value: f64,
    pub low_stock_count: i64,
    pub out_of_stock_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfitLossReport {
    pub date_from: String,
    pub date_to: String,
    pub total_revenue: f64,
    pub total_cost: f64,
    pub gross_profit: f64,
    pub gross_margin_percent: f64,
    pub total_vat_collected: f64,
    pub total_discount_given: f64,
    pub by_category: Vec<CategoryProfit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryProfit {
    pub category_name: String,
    pub quantity_sold: i64,
    pub revenue: f64,
    pub cost: f64,
    pub profit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VatReport {
    pub date_from: String,
    pub date_to: String,
    pub vatable_sales: f64,
    pub vat_exempt_sales: f64,
    pub total_sales: f64,
    pub output_vat: f64,
    pub items: Vec<VatReportItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VatReportItem {
    pub invoice_number: String,
    pub date: String,
    pub customer_name: Option<String>,
    pub customer_tin: Option<String>,
    pub vatable_amount: f64,
    pub vat_amount: f64,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportFilter {
    pub date_from: String,
    pub date_to: String,
}
