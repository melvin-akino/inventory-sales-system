use crate::commands::auth::{require_role, validate_session};
use crate::models::report::*;
use crate::AppState;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub async fn get_dashboard_stats(
    token: String,
    state: State<'_, AppState>,
) -> Result<DashboardStats, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    let total_sales_today: f64 = db
        .query_row(
            "SELECT COALESCE(SUM(total_amount), 0) FROM sales
             WHERE DATE(sale_date) = DATE('now', 'localtime') AND status = 'completed'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let total_transactions_today: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM sales
             WHERE DATE(sale_date) = DATE('now', 'localtime') AND status = 'completed'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let total_products: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM products WHERE is_active = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let low_stock_count: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM products WHERE is_active = 1 AND quantity <= reorder_level",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let total_customers: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM customers WHERE is_active = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let sales_this_month: f64 = db
        .query_row(
            "SELECT COALESCE(SUM(total_amount), 0) FROM sales
             WHERE strftime('%Y-%m', sale_date) = strftime('%Y-%m', 'now', 'localtime')
             AND status = 'completed'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let mut top_stmt = db
        .prepare(
            "SELECT p.name, SUM(si.quantity) as qty_sold, SUM(si.total_price) as revenue
             FROM sale_items si
             JOIN products p ON si.product_id = p.id
             JOIN sales s ON si.sale_id = s.id
             WHERE s.status = 'completed'
             AND strftime('%Y-%m', s.sale_date) = strftime('%Y-%m', 'now', 'localtime')
             GROUP BY p.id ORDER BY qty_sold DESC LIMIT 5",
        )
        .map_err(|e| e.to_string())?;

    let top_products: Vec<TopProduct> = top_stmt
        .query_map([], |row| {
            Ok(TopProduct {
                product_name: row.get(0)?,
                quantity_sold: row.get(1)?,
                total_revenue: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut recent_stmt = db
        .prepare(
            "SELECT s.sale_number, c.name, s.total_amount, s.payment_method, s.sale_date
             FROM sales s
             LEFT JOIN customers c ON s.customer_id = c.id
             WHERE s.status = 'completed'
             ORDER BY s.sale_date DESC LIMIT 10",
        )
        .map_err(|e| e.to_string())?;

    let recent_sales: Vec<RecentSale> = recent_stmt
        .query_map([], |row| {
            Ok(RecentSale {
                sale_number: row.get(0)?,
                customer_name: row.get(1)?,
                total_amount: row.get(2)?,
                payment_method: row.get(3)?,
                sale_date: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(DashboardStats {
        total_sales_today,
        total_transactions_today,
        total_products,
        low_stock_count,
        total_customers,
        sales_this_month,
        top_products,
        recent_sales,
    })
}

#[tauri::command]
pub async fn get_sales_report(
    token: String,
    filter: ReportFilter,
    state: State<'_, AppState>,
) -> Result<SalesReportSummary, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let mut stmt = db
        .prepare(
            "SELECT DATE(s.sale_date), s.sale_number, c.name, u.full_name,
                    s.subtotal, s.discount_amount, s.vat_amount, s.total_amount,
                    s.payment_method, s.status
             FROM sales s
             LEFT JOIN customers c ON s.customer_id = c.id
             JOIN users u ON s.user_id = u.id
             WHERE DATE(s.sale_date) BETWEEN ? AND ?
             ORDER BY s.sale_date",
        )
        .map_err(|e| e.to_string())?;

    let items: Vec<SalesReportItem> = stmt
        .query_map(params![filter.date_from, filter.date_to], |row| {
            Ok(SalesReportItem {
                date: row.get(0)?,
                sale_number: row.get(1)?,
                customer_name: row.get(2)?,
                cashier_name: row.get(3)?,
                subtotal: row.get(4)?,
                discount: row.get(5)?,
                vat: row.get(6)?,
                total: row.get(7)?,
                payment_method: row.get(8)?,
                status: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let total_sales: f64 = items.iter().filter(|i| i.status == "completed").map(|i| i.subtotal).sum();
    let total_vat: f64 = items.iter().filter(|i| i.status == "completed").map(|i| i.vat).sum();
    let total_discount: f64 = items.iter().filter(|i| i.status == "completed").map(|i| i.discount).sum();
    let grand_total: f64 = items.iter().filter(|i| i.status == "completed").map(|i| i.total).sum();
    let transaction_count = items.iter().filter(|i| i.status == "completed").count() as i64;

    Ok(SalesReportSummary {
        items,
        total_sales,
        total_vat,
        total_discount,
        grand_total,
        transaction_count,
    })
}

#[tauri::command]
pub async fn get_inventory_report(
    token: String,
    state: State<'_, AppState>,
) -> Result<InventoryReportSummary, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let mut stmt = db
        .prepare(
            "SELECT p.sku, p.name, c.name, p.unit, p.quantity, p.cost_price,
                    p.selling_price, (p.quantity * p.cost_price), p.reorder_level
             FROM products p
             LEFT JOIN categories c ON p.category_id = c.id
             WHERE p.is_active = 1
             ORDER BY p.name",
        )
        .map_err(|e| e.to_string())?;

    let items: Vec<InventoryReportItem> = stmt
        .query_map([], |row| {
            let qty: i64 = row.get(4)?;
            let reorder: i64 = row.get(8)?;
            let status = if qty == 0 {
                "Out of Stock"
            } else if qty <= reorder {
                "Low Stock"
            } else {
                "In Stock"
            }
            .to_string();
            Ok(InventoryReportItem {
                sku: row.get(0)?,
                product_name: row.get(1)?,
                category: row.get(2)?,
                unit: row.get(3)?,
                quantity: qty,
                cost_price: row.get(5)?,
                selling_price: row.get(6)?,
                inventory_value: row.get(7)?,
                reorder_level: reorder,
                status,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let total_inventory_value: f64 = items.iter().map(|i| i.inventory_value).sum();
    let total_items = items.len() as i64;
    let low_stock_count = items.iter().filter(|i| i.status == "Low Stock").count() as i64;
    let out_of_stock_count = items.iter().filter(|i| i.status == "Out of Stock").count() as i64;

    Ok(InventoryReportSummary {
        items,
        total_items,
        total_inventory_value,
        low_stock_count,
        out_of_stock_count,
    })
}

#[tauri::command]
pub async fn get_profit_loss_report(
    token: String,
    filter: ReportFilter,
    state: State<'_, AppState>,
) -> Result<ProfitLossReport, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let (total_revenue, total_vat, total_discount): (f64, f64, f64) = db
        .query_row(
            "SELECT COALESCE(SUM(subtotal), 0), COALESCE(SUM(vat_amount), 0), COALESCE(SUM(discount_amount), 0)
             FROM sales
             WHERE DATE(sale_date) BETWEEN ? AND ? AND status = 'completed'",
            params![filter.date_from, filter.date_to],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap_or((0.0, 0.0, 0.0));

    let total_cost: f64 = db
        .query_row(
            "SELECT COALESCE(SUM(si.quantity * p.cost_price), 0)
             FROM sale_items si
             JOIN products p ON si.product_id = p.id
             JOIN sales s ON si.sale_id = s.id
             WHERE DATE(s.sale_date) BETWEEN ? AND ? AND s.status = 'completed'",
            params![filter.date_from, filter.date_to],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    let gross_profit = total_revenue - total_cost;
    let gross_margin_percent = if total_revenue > 0.0 {
        (gross_profit / total_revenue) * 100.0
    } else {
        0.0
    };

    let mut cat_stmt = db
        .prepare(
            "SELECT COALESCE(c.name, 'Uncategorized'), SUM(si.quantity),
                    SUM(si.total_price), SUM(si.quantity * p.cost_price)
             FROM sale_items si
             JOIN products p ON si.product_id = p.id
             LEFT JOIN categories c ON p.category_id = c.id
             JOIN sales s ON si.sale_id = s.id
             WHERE DATE(s.sale_date) BETWEEN ? AND ? AND s.status = 'completed'
             GROUP BY c.id ORDER BY SUM(si.total_price) DESC",
        )
        .map_err(|e| e.to_string())?;

    let by_category: Vec<CategoryProfit> = cat_stmt
        .query_map(params![filter.date_from, filter.date_to], |row| {
            let revenue: f64 = row.get(2)?;
            let cost: f64 = row.get(3)?;
            Ok(CategoryProfit {
                category_name: row.get(0)?,
                quantity_sold: row.get(1)?,
                revenue,
                cost,
                profit: revenue - cost,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(ProfitLossReport {
        date_from: filter.date_from,
        date_to: filter.date_to,
        total_revenue,
        total_cost,
        gross_profit,
        gross_margin_percent,
        total_vat_collected: total_vat,
        total_discount_given: total_discount,
        by_category,
    })
}

#[tauri::command]
pub async fn get_vat_report(
    token: String,
    filter: ReportFilter,
    state: State<'_, AppState>,
) -> Result<VatReport, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let mut stmt = db
        .prepare(
            "SELECT i.invoice_number, DATE(s.sale_date), c.name, c.tin_number,
                    s.subtotal, s.vat_amount, s.total_amount
             FROM invoices i
             JOIN sales s ON i.sale_id = s.id
             LEFT JOIN customers c ON s.customer_id = c.id
             WHERE DATE(s.sale_date) BETWEEN ? AND ? AND s.status = 'completed' AND i.status != 'void'
             ORDER BY s.sale_date",
        )
        .map_err(|e| e.to_string())?;

    let items: Vec<VatReportItem> = stmt
        .query_map(params![filter.date_from, filter.date_to], |row| {
            Ok(VatReportItem {
                invoice_number: row.get(0)?,
                date: row.get(1)?,
                customer_name: row.get(2)?,
                customer_tin: row.get(3)?,
                vatable_amount: row.get(4)?,
                vat_amount: row.get(5)?,
                total_amount: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let vatable_sales: f64 = items.iter().map(|i| i.vatable_amount).sum();
    let output_vat: f64 = items.iter().map(|i| i.vat_amount).sum();
    let total_sales: f64 = items.iter().map(|i| i.total_amount).sum();

    Ok(VatReport {
        date_from: filter.date_from,
        date_to: filter.date_to,
        vatable_sales,
        vat_exempt_sales: 0.0,
        total_sales,
        output_vat,
        items,
    })
}
