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

fn generate_sale_number(db: &rusqlite::Connection) -> String {
    let prefix: String = db
        .query_row("SELECT value FROM settings WHERE key = 'sale_prefix'", [], |r| r.get(0))
        .unwrap_or_else(|_| "SL".to_string());
    let count: i64 = db
        .query_row("SELECT COUNT(*) FROM sales", [], |r| r.get(0))
        .unwrap_or(0);
    let date = chrono::Local::now().format("%Y%m%d");
    format!("{}-{}-{:05}", prefix, date, count + 1)
}

fn generate_invoice_number(db: &rusqlite::Connection) -> String {
    let prefix: String = db
        .query_row("SELECT value FROM settings WHERE key = 'invoice_prefix'", [], |r| r.get(0))
        .unwrap_or_else(|_| "OR".to_string());
    let count: i64 = db
        .query_row("SELECT COUNT(*) FROM invoices", [], |r| r.get(0))
        .unwrap_or(0);
    let date = chrono::Local::now().format("%Y%m%d");
    format!("{}-{}-{:06}", prefix, date, count + 1)
}

fn get_sale_by_id(db: &rusqlite::Connection, sale_id: i64) -> Result<Value, (StatusCode, String)> {
    let sale = db
        .query_row(
            "SELECT s.id, s.sale_number, s.customer_id, c.name, s.user_id, u.full_name,
                    s.sale_date, s.subtotal, s.discount_amount, s.vat_amount, s.total_amount,
                    s.amount_paid, s.change_amount, s.payment_method, s.status, s.notes, s.created_at
             FROM sales s
             LEFT JOIN customers c ON s.customer_id = c.id
             JOIN users u ON s.user_id = u.id
             WHERE s.id = ?",
            params![sale_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, f64>(7)?,
                    row.get::<_, f64>(8)?,
                    row.get::<_, f64>(9)?,
                    row.get::<_, f64>(10)?,
                    row.get::<_, f64>(11)?,
                    row.get::<_, f64>(12)?,
                    row.get::<_, String>(13)?,
                    row.get::<_, String>(14)?,
                    row.get::<_, Option<String>>(15)?,
                    row.get::<_, String>(16)?,
                ))
            },
        )
        .map_err(|_| bad_req("Sale not found"))?;

    let mut stmt = db
        .prepare(
            "SELECT id, sale_id, product_id, product_name, quantity, unit_price,
                    discount_percent, vat_amount, total_price
             FROM sale_items WHERE sale_id = ?",
        )
        .map_err(|e| server_err(e.to_string()))?;

    let items: Vec<Value> = stmt
        .query_map(params![sale.0], |row| {
            Ok(json!({
                "id":               row.get::<_, i64>(0)?,
                "sale_id":          row.get::<_, i64>(1)?,
                "product_id":       row.get::<_, i64>(2)?,
                "product_name":     row.get::<_, String>(3)?,
                "quantity":         row.get::<_, i64>(4)?,
                "unit_price":       row.get::<_, f64>(5)?,
                "discount_percent": row.get::<_, f64>(6)?,
                "vat_amount":       row.get::<_, f64>(7)?,
                "total_price":      row.get::<_, f64>(8)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({
        "id":             sale.0,
        "sale_number":    sale.1,
        "customer_id":    sale.2,
        "customer_name":  sale.3,
        "user_id":        sale.4,
        "cashier_name":   sale.5,
        "sale_date":      sale.6,
        "subtotal":       sale.7,
        "discount_amount":sale.8,
        "vat_amount":     sale.9,
        "total_amount":   sale.10,
        "amount_paid":    sale.11,
        "change_amount":  sale.12,
        "payment_method": sale.13,
        "status":         sale.14,
        "notes":          sale.15,
        "created_at":     sale.16,
        "items":          items
    }))
}

pub async fn create_sale(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager", "cashier"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let customer_id = req["customer_id"].as_i64().or_else(|| req["customerId"].as_i64());
    let items_json = req["items"].as_array().ok_or_else(|| bad_req("Missing items"))?;

    if items_json.is_empty() {
        return Err(bad_req("Sale must have at least one item"));
    }

    let discount_amount = req["discount_amount"].as_f64()
        .or_else(|| req["discountAmount"].as_f64())
        .unwrap_or(0.0);
    let amount_paid = req["amount_paid"].as_f64()
        .or_else(|| req["amountPaid"].as_f64())
        .ok_or_else(|| bad_req("Missing amount_paid"))?;
    let payment_method = req["payment_method"].as_str()
        .or_else(|| req["paymentMethod"].as_str())
        .ok_or_else(|| bad_req("Missing payment_method"))?;
    let notes = req["notes"].as_str();

    // Parse items
    struct ItemInput {
        product_id: i64,
        quantity: i64,
        unit_price: f64,
        discount_percent: f64,
    }

    let mut item_inputs: Vec<ItemInput> = Vec::new();
    for item in items_json {
        let product_id = item["product_id"].as_i64()
            .or_else(|| item["productId"].as_i64())
            .ok_or_else(|| bad_req("Item missing product_id"))?;
        let quantity = item["quantity"].as_i64().ok_or_else(|| bad_req("Item missing quantity"))?;
        let unit_price = item["unit_price"].as_f64()
            .or_else(|| item["unitPrice"].as_f64())
            .ok_or_else(|| bad_req("Item missing unit_price"))?;
        let discount_percent = item["discount_percent"].as_f64()
            .or_else(|| item["discountPercent"].as_f64())
            .unwrap_or(0.0);

        if quantity <= 0 {
            return Err(bad_req("Item quantity must be greater than 0"));
        }

        item_inputs.push(ItemInput { product_id, quantity, unit_price, discount_percent });
    }

    let vat_rate: f64 = db
        .query_row("SELECT CAST(value AS REAL) FROM settings WHERE key = 'vat_rate'", [], |r| r.get(0))
        .unwrap_or(12.0)
        / 100.0;

    // Validate stock and compute totals
    let mut computed: Vec<(i64, String, i64, f64, f64, f64, f64)> = Vec::new();
    let mut subtotal = 0.0f64;
    let mut total_vat = 0.0f64;

    for item in &item_inputs {
        let (qty_in_stock, prod_name, is_vat_exempt): (i64, String, bool) = db
            .query_row(
                "SELECT quantity, name, is_vat_exempt FROM products WHERE id = ? AND is_active = 1",
                params![item.product_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|_| bad_req(format!("Product ID {} not found", item.product_id)))?;

        if qty_in_stock < item.quantity {
            return Err(bad_req(format!(
                "Insufficient stock for '{}'. Available: {}, Requested: {}",
                prod_name, qty_in_stock, item.quantity
            )));
        }

        let line_sub = item.unit_price * item.quantity as f64;
        let disc_amt = line_sub * (item.discount_percent / 100.0);
        let taxable  = line_sub - disc_amt;
        let vat      = if is_vat_exempt { 0.0 } else { taxable * vat_rate };
        let total    = taxable + vat;

        subtotal  += taxable;
        total_vat += vat;
        computed.push((item.product_id, prod_name, item.quantity, item.unit_price, item.discount_percent, vat, total));
    }

    let gross_total = subtotal + total_vat - discount_amount;
    let change = amount_paid - gross_total;

    if amount_paid < gross_total {
        return Err(bad_req(format!(
            "Amount paid (₱{:.2}) is less than total (₱{:.2})",
            amount_paid, gross_total
        )));
    }

    let sale_number = generate_sale_number(&db);

    db.execute(
        "INSERT INTO sales (sale_number, customer_id, user_id, subtotal, discount_amount, vat_amount, total_amount, amount_paid, change_amount, payment_method, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![sale_number, customer_id, session.user_id, subtotal, discount_amount, total_vat, gross_total, amount_paid, change, payment_method, notes],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let sale_id = db.last_insert_rowid();

    for (prod_id, prod_name, qty, unit_price, disc_pct, vat, total) in &computed {
        db.execute(
            "INSERT INTO sale_items (sale_id, product_id, product_name, quantity, unit_price, discount_percent, vat_amount, total_price)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![sale_id, prod_id, prod_name, qty, unit_price, disc_pct, vat, total],
        )
        .map_err(|e| server_err(e.to_string()))?;

        db.execute(
            "UPDATE products SET quantity = quantity - ?, updated_at = datetime('now') WHERE id = ?",
            params![qty, prod_id],
        )
        .map_err(|e| server_err(e.to_string()))?;
    }

    let invoice_number = generate_invoice_number(&db);
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    db.execute(
        "INSERT INTO invoices (sale_id, invoice_number, invoice_date, status) VALUES (?, ?, ?, 'paid')",
        params![sale_id, invoice_number, today],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let sale = get_sale_by_id(&db, sale_id)?;
    Ok(Json(sale))
}

pub async fn get_sale(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    let sale = get_sale_by_id(&db, id)?;
    Ok(Json(sale))
}

pub async fn get_sales(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;

    let filter = body.get("filter");
    let cashier_filter = if session.role == "cashier" {
        format!("AND s.user_id = {}", session.user_id)
    } else {
        String::new()
    };

    let mut sql = format!(
        "SELECT s.id, s.sale_number, s.customer_id, c.name, s.user_id, u.full_name,
                s.sale_date, s.subtotal, s.discount_amount, s.vat_amount, s.total_amount,
                s.amount_paid, s.change_amount, s.payment_method, s.status, s.notes, s.created_at
         FROM sales s
         LEFT JOIN customers c ON s.customer_id = c.id
         JOIN users u ON s.user_id = u.id
         WHERE 1=1 {}",
        cashier_filter
    );

    if let Some(f) = filter {
        if let Some(d) = f["date_from"].as_str().or_else(|| f["dateFrom"].as_str()) {
            sql.push_str(&format!(" AND DATE(s.sale_date) >= '{}'", d.replace('\'', "")));
        }
        if let Some(d) = f["date_to"].as_str().or_else(|| f["dateTo"].as_str()) {
            sql.push_str(&format!(" AND DATE(s.sale_date) <= '{}'", d.replace('\'', "")));
        }
        if let Some(pm) = f["payment_method"].as_str().or_else(|| f["paymentMethod"].as_str()) {
            sql.push_str(&format!(" AND s.payment_method = '{}'", pm.replace('\'', "")));
        }
        if let Some(st) = f["status"].as_str() {
            sql.push_str(&format!(" AND s.status = '{}'", st.replace('\'', "")));
        }
        if let Some(cid) = f["customer_id"].as_i64().or_else(|| f["customerId"].as_i64()) {
            sql.push_str(&format!(" AND s.customer_id = {}", cid));
        }
        if let Some(search) = f["search"].as_str() {
            let s = search.replace('\'', "");
            sql.push_str(&format!(" AND (s.sale_number LIKE '%{s}%' OR c.name LIKE '%{s}%')"));
        }
    }

    sql.push_str(" ORDER BY s.sale_date DESC LIMIT 500");

    let mut stmt = db.prepare(&sql).map_err(|e| server_err(e.to_string()))?;
    let sales: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id":             row.get::<_, i64>(0)?,
                "sale_number":    row.get::<_, String>(1)?,
                "customer_id":    row.get::<_, Option<i64>>(2)?,
                "customer_name":  row.get::<_, Option<String>>(3)?,
                "user_id":        row.get::<_, i64>(4)?,
                "cashier_name":   row.get::<_, String>(5)?,
                "sale_date":      row.get::<_, String>(6)?,
                "subtotal":       row.get::<_, f64>(7)?,
                "discount_amount":row.get::<_, f64>(8)?,
                "vat_amount":     row.get::<_, f64>(9)?,
                "total_amount":   row.get::<_, f64>(10)?,
                "amount_paid":    row.get::<_, f64>(11)?,
                "change_amount":  row.get::<_, f64>(12)?,
                "payment_method": row.get::<_, String>(13)?,
                "status":         row.get::<_, String>(14)?,
                "notes":          row.get::<_, Option<String>>(15)?,
                "created_at":     row.get::<_, String>(16)?,
                "items":          []
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(sales)))
}

pub async fn void_sale(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    let reason = body["reason"].as_str().unwrap_or("No reason provided");

    let current_status: String = db
        .query_row("SELECT status FROM sales WHERE id = ?", params![id], |r| r.get(0))
        .map_err(|_| bad_req("Sale not found"))?;

    if current_status != "completed" {
        return Err(bad_req("Only completed sales can be voided"));
    }

    // Restore inventory
    let mut stmt = db
        .prepare("SELECT product_id, quantity FROM sale_items WHERE sale_id = ?")
        .map_err(|e| server_err(e.to_string()))?;
    let items: Vec<(i64, i64)> = stmt
        .query_map(params![id], |r| Ok((r.get(0)?, r.get(1)?)))
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    for (prod_id, qty) in items {
        db.execute(
            "UPDATE products SET quantity = quantity + ?, updated_at = datetime('now') WHERE id = ?",
            params![qty, prod_id],
        )
        .map_err(|e| server_err(e.to_string()))?;
    }

    db.execute(
        "UPDATE sales SET status = 'void', notes = ? WHERE id = ?",
        params![format!("VOIDED: {}", reason), id],
    )
    .map_err(|e| server_err(e.to_string()))?;

    db.execute("UPDATE invoices SET status = 'void' WHERE sale_id = ?", params![id]).ok();

    let sale = get_sale_by_id(&db, id)?;
    Ok(Json(sale))
}
