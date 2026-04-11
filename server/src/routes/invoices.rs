use crate::{
    auth_guard::validate_session,
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};

use super::auth::get_token;

fn row_to_invoice(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    Ok(json!({
        "id":               row.get::<_, i64>(0)?,
        "sale_id":          row.get::<_, i64>(1)?,
        "sale_number":      row.get::<_, String>(2)?,
        "invoice_number":   row.get::<_, String>(3)?,
        "invoice_date":     row.get::<_, String>(4)?,
        "due_date":         row.get::<_, Option<String>>(5)?,
        "customer_name":    row.get::<_, Option<String>>(6)?,
        "customer_tin":     row.get::<_, Option<String>>(7)?,
        "customer_address": row.get::<_, Option<String>>(8)?,
        "status":           row.get::<_, String>(9)?,
        "total_amount":     row.get::<_, f64>(10)?,
        "vat_amount":       row.get::<_, f64>(11)?,
        "notes":            row.get::<_, Option<String>>(12)?,
        "created_at":       row.get::<_, String>(13)?
    }))
}

const INVOICE_SELECT: &str =
    "SELECT i.id, i.sale_id, s.sale_number, i.invoice_number, i.invoice_date,
            i.due_date, c.name, c.tin_number, c.address,
            i.status, s.total_amount, s.vat_amount, i.notes, i.created_at
     FROM invoices i
     JOIN sales s ON i.sale_id = s.id
     LEFT JOIN customers c ON s.customer_id = c.id";

pub async fn get_invoices(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let filter = body.get("filter");
    let mut sql = format!("{} WHERE 1=1", INVOICE_SELECT);

    if let Some(f) = filter {
        if let Some(d) = f["date_from"].as_str().or_else(|| f["dateFrom"].as_str()) {
            sql.push_str(&format!(" AND i.invoice_date >= '{}'", d.replace('\'', "")));
        }
        if let Some(d) = f["date_to"].as_str().or_else(|| f["dateTo"].as_str()) {
            sql.push_str(&format!(" AND i.invoice_date <= '{}'", d.replace('\'', "")));
        }
        if let Some(st) = f["status"].as_str() {
            sql.push_str(&format!(" AND i.status = '{}'", st.replace('\'', "")));
        }
        if let Some(search) = f["search"].as_str() {
            let s = search.replace('\'', "");
            sql.push_str(&format!(
                " AND (i.invoice_number LIKE '%{s}%' OR s.sale_number LIKE '%{s}%' OR c.name LIKE '%{s}%')"
            ));
        }
    }

    sql.push_str(" ORDER BY i.created_at DESC LIMIT 500");

    let mut stmt = db.prepare(&sql).map_err(|e| server_err(e.to_string()))?;
    let invoices: Vec<Value> = stmt
        .query_map([], row_to_invoice)
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(invoices)))
}

pub async fn get_invoice(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;

    let invoice = db
        .query_row(
            &format!("{} WHERE i.id = ?", INVOICE_SELECT),
            params![id],
            row_to_invoice,
        )
        .map_err(|_| bad_req("Invoice not found"))?;

    Ok(Json(invoice))
}
