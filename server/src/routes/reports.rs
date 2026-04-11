use crate::{
    auth_guard::{require_role, validate_session},
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};

use super::auth::get_token;

pub async fn get_dashboard_stats(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
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
        .query_row("SELECT COUNT(*) FROM products WHERE is_active = 1", [], |r| r.get(0))
        .unwrap_or(0);

    let low_stock_count: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM products WHERE is_active = 1 AND quantity <= reorder_level",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let total_customers: i64 = db
        .query_row("SELECT COUNT(*) FROM customers WHERE is_active = 1", [], |r| r.get(0))
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
        .map_err(|e| server_err(e.to_string()))?;

    let top_products: Vec<Value> = top_stmt
        .query_map([], |row| {
            Ok(json!({
                "product_name":   row.get::<_, String>(0)?,
                "quantity_sold":  row.get::<_, i64>(1)?,
                "total_revenue":  row.get::<_, f64>(2)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
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
        .map_err(|e| server_err(e.to_string()))?;

    let recent_sales: Vec<Value> = recent_stmt
        .query_map([], |row| {
            Ok(json!({
                "sale_number":    row.get::<_, String>(0)?,
                "customer_name":  row.get::<_, Option<String>>(1)?,
                "total_amount":   row.get::<_, f64>(2)?,
                "payment_method": row.get::<_, String>(3)?,
                "sale_date":      row.get::<_, String>(4)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!({
        "total_sales_today":       total_sales_today,
        "total_transactions_today":total_transactions_today,
        "total_products":          total_products,
        "low_stock_count":         low_stock_count,
        "total_customers":         total_customers,
        "sales_this_month":        sales_this_month,
        "top_products":            top_products,
        "recent_sales":            recent_sales
    })))
}

pub async fn get_sales_report(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let filter = body.get("filter").ok_or_else(|| bad_req("Missing filter"))?;
    let date_from = filter["date_from"].as_str()
        .or_else(|| filter["dateFrom"].as_str())
        .ok_or_else(|| bad_req("Missing date_from"))?;
    let date_to = filter["date_to"].as_str()
        .or_else(|| filter["dateTo"].as_str())
        .ok_or_else(|| bad_req("Missing date_to"))?;

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
        .map_err(|e| server_err(e.to_string()))?;

    let items: Vec<Value> = stmt
        .query_map(params![date_from, date_to], |row| {
            Ok(json!({
                "date":           row.get::<_, String>(0)?,
                "sale_number":    row.get::<_, String>(1)?,
                "customer_name":  row.get::<_, Option<String>>(2)?,
                "cashier_name":   row.get::<_, String>(3)?,
                "subtotal":       row.get::<_, f64>(4)?,
                "discount":       row.get::<_, f64>(5)?,
                "vat":            row.get::<_, f64>(6)?,
                "total":          row.get::<_, f64>(7)?,
                "payment_method": row.get::<_, String>(8)?,
                "status":         row.get::<_, String>(9)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    let total_sales: f64 = items.iter()
        .filter(|i| i["status"].as_str() == Some("completed"))
        .filter_map(|i| i["subtotal"].as_f64())
        .sum();
    let total_vat: f64 = items.iter()
        .filter(|i| i["status"].as_str() == Some("completed"))
        .filter_map(|i| i["vat"].as_f64())
        .sum();
    let total_discount: f64 = items.iter()
        .filter(|i| i["status"].as_str() == Some("completed"))
        .filter_map(|i| i["discount"].as_f64())
        .sum();
    let grand_total: f64 = items.iter()
        .filter(|i| i["status"].as_str() == Some("completed"))
        .filter_map(|i| i["total"].as_f64())
        .sum();
    let transaction_count = items.iter()
        .filter(|i| i["status"].as_str() == Some("completed"))
        .count() as i64;

    Ok(Json(json!({
        "items":             items,
        "total_sales":       total_sales,
        "total_vat":         total_vat,
        "total_discount":    total_discount,
        "grand_total":       grand_total,
        "transaction_count": transaction_count
    })))
}

pub async fn get_inventory_report(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
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
        .map_err(|e| server_err(e.to_string()))?;

    let items: Vec<Value> = stmt
        .query_map([], |row| {
            let qty: i64 = row.get(4)?;
            let reorder: i64 = row.get(8)?;
            let status = if qty == 0 { "Out of Stock" } else if qty <= reorder { "Low Stock" } else { "In Stock" };
            Ok(json!({
                "sku":             row.get::<_, String>(0)?,
                "product_name":    row.get::<_, String>(1)?,
                "category":        row.get::<_, Option<String>>(2)?,
                "unit":            row.get::<_, String>(3)?,
                "quantity":        qty,
                "cost_price":      row.get::<_, f64>(5)?,
                "selling_price":   row.get::<_, f64>(6)?,
                "inventory_value": row.get::<_, f64>(7)?,
                "reorder_level":   reorder,
                "status":          status
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    let total_inventory_value: f64 = items.iter()
        .filter_map(|i| i["inventory_value"].as_f64())
        .sum();
    let total_items = items.len() as i64;
    let low_stock_count = items.iter()
        .filter(|i| i["status"].as_str() == Some("Low Stock"))
        .count() as i64;
    let out_of_stock_count = items.iter()
        .filter(|i| i["status"].as_str() == Some("Out of Stock"))
        .count() as i64;

    Ok(Json(json!({
        "items":                 items,
        "total_items":           total_items,
        "total_inventory_value": total_inventory_value,
        "low_stock_count":       low_stock_count,
        "out_of_stock_count":    out_of_stock_count
    })))
}

pub async fn get_profit_loss_report(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let filter = body.get("filter").ok_or_else(|| bad_req("Missing filter"))?;
    let date_from = filter["date_from"].as_str()
        .or_else(|| filter["dateFrom"].as_str())
        .ok_or_else(|| bad_req("Missing date_from"))?;
    let date_to = filter["date_to"].as_str()
        .or_else(|| filter["dateTo"].as_str())
        .ok_or_else(|| bad_req("Missing date_to"))?;

    let (total_revenue, total_vat, total_discount): (f64, f64, f64) = db
        .query_row(
            "SELECT COALESCE(SUM(subtotal), 0), COALESCE(SUM(vat_amount), 0), COALESCE(SUM(discount_amount), 0)
             FROM sales
             WHERE DATE(sale_date) BETWEEN ? AND ? AND status = 'completed'",
            params![date_from, date_to],
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
            params![date_from, date_to],
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
        .map_err(|e| server_err(e.to_string()))?;

    let by_category: Vec<Value> = cat_stmt
        .query_map(params![date_from, date_to], |row| {
            let revenue: f64 = row.get(2)?;
            let cost: f64 = row.get(3)?;
            Ok(json!({
                "category_name":  row.get::<_, String>(0)?,
                "quantity_sold":  row.get::<_, i64>(1)?,
                "revenue":        revenue,
                "cost":           cost,
                "profit":         revenue - cost
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!({
        "date_from":            date_from,
        "date_to":              date_to,
        "total_revenue":        total_revenue,
        "total_cost":           total_cost,
        "gross_profit":         gross_profit,
        "gross_margin_percent": gross_margin_percent,
        "total_vat_collected":  total_vat,
        "total_discount_given": total_discount,
        "by_category":          by_category
    })))
}

pub async fn get_vat_report(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let filter = body.get("filter").ok_or_else(|| bad_req("Missing filter"))?;
    let date_from = filter["date_from"].as_str()
        .or_else(|| filter["dateFrom"].as_str())
        .ok_or_else(|| bad_req("Missing date_from"))?;
    let date_to = filter["date_to"].as_str()
        .or_else(|| filter["dateTo"].as_str())
        .ok_or_else(|| bad_req("Missing date_to"))?;

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
        .map_err(|e| server_err(e.to_string()))?;

    let items: Vec<Value> = stmt
        .query_map(params![date_from, date_to], |row| {
            Ok(json!({
                "invoice_number":  row.get::<_, String>(0)?,
                "date":            row.get::<_, String>(1)?,
                "customer_name":   row.get::<_, Option<String>>(2)?,
                "customer_tin":    row.get::<_, Option<String>>(3)?,
                "vatable_amount":  row.get::<_, f64>(4)?,
                "vat_amount":      row.get::<_, f64>(5)?,
                "total_amount":    row.get::<_, f64>(6)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    let vatable_sales: f64 = items.iter().filter_map(|i| i["vatable_amount"].as_f64()).sum();
    let output_vat: f64 = items.iter().filter_map(|i| i["vat_amount"].as_f64()).sum();
    let total_sales: f64 = items.iter().filter_map(|i| i["total_amount"].as_f64()).sum();

    Ok(Json(json!({
        "date_from":        date_from,
        "date_to":          date_to,
        "vatable_sales":    vatable_sales,
        "vat_exempt_sales": 0.0,
        "total_sales":      total_sales,
        "output_vat":       output_vat,
        "items":            items
    })))
}
