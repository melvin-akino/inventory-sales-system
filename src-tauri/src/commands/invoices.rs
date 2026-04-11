use crate::commands::auth::validate_session;
use crate::models::invoice::{Invoice, InvoiceFilter};
use crate::AppState;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub async fn get_invoices(
    token: String,
    filter: Option<InvoiceFilter>,
    state: State<'_, AppState>,
) -> Result<Vec<Invoice>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    let f = filter.unwrap_or(InvoiceFilter {
        date_from: None,
        date_to: None,
        status: None,
        search: None,
    });

    let mut sql = "SELECT i.id, i.sale_id, s.sale_number, i.invoice_number, i.invoice_date,
                          i.due_date, c.name, c.tin_number, c.address,
                          i.status, s.total_amount, s.vat_amount, i.notes, i.created_at
                   FROM invoices i
                   JOIN sales s ON i.sale_id = s.id
                   LEFT JOIN customers c ON s.customer_id = c.id
                   WHERE 1=1"
        .to_string();

    if let Some(ref d) = f.date_from {
        sql.push_str(&format!(" AND i.invoice_date >= '{}'", d.replace('\'', "")));
    }
    if let Some(ref d) = f.date_to {
        sql.push_str(&format!(" AND i.invoice_date <= '{}'", d.replace('\'', "")));
    }
    if let Some(ref st) = f.status {
        sql.push_str(&format!(" AND i.status = '{}'", st.replace('\'', "")));
    }
    if let Some(ref search) = f.search {
        let s = search.replace('\'', "");
        sql.push_str(&format!(
            " AND (i.invoice_number LIKE '%{s}%' OR s.sale_number LIKE '%{s}%' OR c.name LIKE '%{s}%')"
        ));
    }

    sql.push_str(" ORDER BY i.created_at DESC LIMIT 500");

    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;
    let invoices = stmt
        .query_map([], |row| {
            Ok(Invoice {
                id: row.get(0)?,
                sale_id: row.get(1)?,
                sale_number: row.get(2)?,
                invoice_number: row.get(3)?,
                invoice_date: row.get(4)?,
                due_date: row.get(5)?,
                customer_name: row.get(6)?,
                customer_tin: row.get(7)?,
                customer_address: row.get(8)?,
                status: row.get(9)?,
                total_amount: row.get(10)?,
                vat_amount: row.get(11)?,
                notes: row.get(12)?,
                created_at: row.get(13)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(invoices)
}

#[tauri::command]
pub async fn get_invoice(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<Invoice, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    db.query_row(
        "SELECT i.id, i.sale_id, s.sale_number, i.invoice_number, i.invoice_date,
                i.due_date, c.name, c.tin_number, c.address,
                i.status, s.total_amount, s.vat_amount, i.notes, i.created_at
         FROM invoices i
         JOIN sales s ON i.sale_id = s.id
         LEFT JOIN customers c ON s.customer_id = c.id
         WHERE i.id = ?",
        params![id],
        |row| {
            Ok(Invoice {
                id: row.get(0)?,
                sale_id: row.get(1)?,
                sale_number: row.get(2)?,
                invoice_number: row.get(3)?,
                invoice_date: row.get(4)?,
                due_date: row.get(5)?,
                customer_name: row.get(6)?,
                customer_tin: row.get(7)?,
                customer_address: row.get(8)?,
                status: row.get(9)?,
                total_amount: row.get(10)?,
                vat_amount: row.get(11)?,
                notes: row.get(12)?,
                created_at: row.get(13)?,
            })
        },
    )
    .map_err(|_| "Invoice not found".to_string())
}
