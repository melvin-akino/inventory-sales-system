use crate::commands::auth::{require_role, validate_session};
use crate::models::sale::{CreateSaleRequest, Sale, SaleFilter, SaleItem};
use crate::AppState;
use rusqlite::params;
use tauri::State;

fn generate_sale_number(db: &rusqlite::Connection) -> Result<String, String> {
    let prefix: String = db
        .query_row(
            "SELECT value FROM settings WHERE key = 'sale_prefix'",
            [],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "SL".to_string());

    let count: i64 = db
        .query_row("SELECT COUNT(*) FROM sales", [], |r| r.get(0))
        .unwrap_or(0);

    let date = chrono::Local::now().format("%Y%m%d");
    Ok(format!("{}-{}-{:05}", prefix, date, count + 1))
}

fn generate_invoice_number(db: &rusqlite::Connection) -> Result<String, String> {
    let prefix: String = db
        .query_row(
            "SELECT value FROM settings WHERE key = 'invoice_prefix'",
            [],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "OR".to_string());

    let count: i64 = db
        .query_row("SELECT COUNT(*) FROM invoices", [], |r| r.get(0))
        .unwrap_or(0);

    let date = chrono::Local::now().format("%Y%m%d");
    Ok(format!("{}-{}-{:06}", prefix, date, count + 1))
}

#[tauri::command]
pub async fn create_sale(
    token: String,
    request: CreateSaleRequest,
    state: State<'_, AppState>,
) -> Result<Sale, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager", "cashier"])?;

    if request.items.is_empty() {
        return Err("Sale must have at least one item".to_string());
    }

    let vat_rate: f64 = db
        .query_row(
            "SELECT CAST(value AS REAL) FROM settings WHERE key = 'vat_rate'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(12.0)
        / 100.0;

    // Validate stock and compute totals
    let mut computed_items: Vec<(i64, String, i64, f64, f64, f64, f64)> = Vec::new();
    let mut subtotal = 0.0f64;
    let mut total_vat = 0.0f64;

    for item in &request.items {
        if item.quantity <= 0 {
            return Err("Item quantity must be greater than 0".to_string());
        }

        let (qty_in_stock, prod_name, is_vat_exempt): (i64, String, bool) = db
            .query_row(
                "SELECT quantity, name, is_vat_exempt FROM products WHERE id = ? AND is_active = 1",
                params![item.product_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|_| format!("Product ID {} not found", item.product_id))?;

        if qty_in_stock < item.quantity {
            return Err(format!(
                "Insufficient stock for '{}'. Available: {}, Requested: {}",
                prod_name, qty_in_stock, item.quantity
            ));
        }

        let line_subtotal = item.unit_price * item.quantity as f64;
        let discount_amt = line_subtotal * (item.discount_percent / 100.0);
        let taxable = line_subtotal - discount_amt;
        let vat = if is_vat_exempt { 0.0 } else { taxable * vat_rate };
        let total = taxable + vat;

        subtotal += taxable;
        total_vat += vat;
        computed_items.push((
            item.product_id,
            prod_name,
            item.quantity,
            item.unit_price,
            item.discount_percent,
            vat,
            total,
        ));
    }

    let gross_total = subtotal + total_vat - request.discount_amount;
    let change = request.amount_paid - gross_total;
    if request.amount_paid < gross_total {
        return Err(format!(
            "Amount paid (₱{:.2}) is less than total (₱{:.2})",
            request.amount_paid, gross_total
        ));
    }

    let sale_number = generate_sale_number(&db)?;

    db.execute(
        "INSERT INTO sales (sale_number, customer_id, user_id, subtotal, discount_amount, vat_amount, total_amount, amount_paid, change_amount, payment_method, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            sale_number,
            request.customer_id,
            session.user_id,
            subtotal,
            request.discount_amount,
            total_vat,
            gross_total,
            request.amount_paid,
            change,
            request.payment_method,
            request.notes
        ],
    )
    .map_err(|e| e.to_string())?;

    let sale_id = db.last_insert_rowid();

    // Insert items and deduct stock
    for (prod_id, prod_name, qty, unit_price, disc_pct, vat, total) in &computed_items {
        db.execute(
            "INSERT INTO sale_items (sale_id, product_id, product_name, quantity, unit_price, discount_percent, vat_amount, total_price)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![sale_id, prod_id, prod_name, qty, unit_price, disc_pct, vat, total],
        )
        .map_err(|e| e.to_string())?;

        db.execute(
            "UPDATE products SET quantity = quantity - ?, updated_at = datetime('now') WHERE id = ?",
            params![qty, prod_id],
        )
        .map_err(|e| e.to_string())?;
    }

    // Auto-generate invoice / official receipt
    let invoice_number = generate_invoice_number(&db)?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    db.execute(
        "INSERT INTO invoices (sale_id, invoice_number, invoice_date, status) VALUES (?, ?, ?, 'paid')",
        params![sale_id, invoice_number, today],
    )
    .map_err(|e| e.to_string())?;

    get_sale_by_id(&db, sale_id)
}

pub fn get_sale_by_id(db: &rusqlite::Connection, sale_id: i64) -> Result<Sale, String> {
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
                Ok(Sale {
                    id: row.get(0)?,
                    sale_number: row.get(1)?,
                    customer_id: row.get(2)?,
                    customer_name: row.get(3)?,
                    user_id: row.get(4)?,
                    cashier_name: row.get(5)?,
                    sale_date: row.get(6)?,
                    subtotal: row.get(7)?,
                    discount_amount: row.get(8)?,
                    vat_amount: row.get(9)?,
                    total_amount: row.get(10)?,
                    amount_paid: row.get(11)?,
                    change_amount: row.get(12)?,
                    payment_method: row.get(13)?,
                    status: row.get(14)?,
                    notes: row.get(15)?,
                    items: vec![],
                    created_at: row.get(16)?,
                })
            },
        )
        .map_err(|_| "Sale not found".to_string())?;

    let mut stmt = db
        .prepare(
            "SELECT id, sale_id, product_id, product_name, quantity, unit_price,
                    discount_percent, vat_amount, total_price
             FROM sale_items WHERE sale_id = ?",
        )
        .map_err(|e| e.to_string())?;

    let items: Vec<SaleItem> = stmt
        .query_map(params![sale.id], |row| {
            Ok(SaleItem {
                id: row.get(0)?,
                sale_id: row.get(1)?,
                product_id: row.get(2)?,
                product_name: row.get(3)?,
                quantity: row.get(4)?,
                unit_price: row.get(5)?,
                discount_percent: row.get(6)?,
                vat_amount: row.get(7)?,
                total_price: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Sale { items, ..sale })
}

#[tauri::command]
pub async fn get_sale(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<Sale, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;
    get_sale_by_id(&db, id)
}

#[tauri::command]
pub async fn get_sales(
    token: String,
    filter: Option<SaleFilter>,
    state: State<'_, AppState>,
) -> Result<Vec<Sale>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;

    let f = filter.unwrap_or(SaleFilter {
        date_from: None,
        date_to: None,
        payment_method: None,
        status: None,
        customer_id: None,
        search: None,
    });

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

    if let Some(ref d) = f.date_from {
        sql.push_str(&format!(" AND DATE(s.sale_date) >= '{}'", d.replace('\'', "")));
    }
    if let Some(ref d) = f.date_to {
        sql.push_str(&format!(" AND DATE(s.sale_date) <= '{}'", d.replace('\'', "")));
    }
    if let Some(ref pm) = f.payment_method {
        sql.push_str(&format!(" AND s.payment_method = '{}'", pm.replace('\'', "")));
    }
    if let Some(ref st) = f.status {
        sql.push_str(&format!(" AND s.status = '{}'", st.replace('\'', "")));
    }
    if let Some(cid) = f.customer_id {
        sql.push_str(&format!(" AND s.customer_id = {}", cid));
    }
    if let Some(ref search) = f.search {
        let s = search.replace('\'', "");
        sql.push_str(&format!(
            " AND (s.sale_number LIKE '%{s}%' OR c.name LIKE '%{s}%')"
        ));
    }

    sql.push_str(" ORDER BY s.sale_date DESC LIMIT 500");

    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;
    let sales: Vec<Sale> = stmt
        .query_map([], |row| {
            Ok(Sale {
                id: row.get(0)?,
                sale_number: row.get(1)?,
                customer_id: row.get(2)?,
                customer_name: row.get(3)?,
                user_id: row.get(4)?,
                cashier_name: row.get(5)?,
                sale_date: row.get(6)?,
                subtotal: row.get(7)?,
                discount_amount: row.get(8)?,
                vat_amount: row.get(9)?,
                total_amount: row.get(10)?,
                amount_paid: row.get(11)?,
                change_amount: row.get(12)?,
                payment_method: row.get(13)?,
                status: row.get(14)?,
                notes: row.get(15)?,
                items: vec![],
                created_at: row.get(16)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(sales)
}

#[tauri::command]
pub async fn void_sale(
    token: String,
    id: i64,
    reason: String,
    state: State<'_, AppState>,
) -> Result<Sale, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let current_status: String = db
        .query_row(
            "SELECT status FROM sales WHERE id = ?",
            params![id],
            |r| r.get(0),
        )
        .map_err(|_| "Sale not found".to_string())?;

    if current_status != "completed" {
        return Err("Only completed sales can be voided".to_string());
    }

    // Restore inventory
    let items = {
        let mut stmt = db
            .prepare("SELECT product_id, quantity FROM sale_items WHERE sale_id = ?")
            .map_err(|e| e.to_string())?;
        let items: Vec<(i64, i64)> = stmt
            .query_map(params![id], |r| Ok((r.get(0)?, r.get(1)?)))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();
        items
    };

    for (prod_id, qty) in items {
        db.execute(
            "UPDATE products SET quantity = quantity + ?, updated_at = datetime('now') WHERE id = ?",
            params![qty, prod_id],
        )
        .map_err(|e| e.to_string())?;
    }

    db.execute(
        "UPDATE sales SET status = 'void', notes = ? WHERE id = ?",
        params![format!("VOIDED: {}", reason), id],
    )
    .map_err(|e| e.to_string())?;

    db.execute(
        "UPDATE invoices SET status = 'void' WHERE sale_id = ?",
        params![id],
    )
    .ok();

    get_sale_by_id(&db, id)
}
