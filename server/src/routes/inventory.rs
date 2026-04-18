use crate::{
    auth_guard::{require_role, validate_session},
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};

use super::auth::{get_token, get_user_industry_id};

// ── Categories ────────────────────────────────────────────────────────────────

pub async fn get_categories(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;
    let industry_id = get_user_industry_id(&db, &token)?;

    let sql = if let Some(ind_id) = industry_id {
        format!("SELECT id, name, description, created_at FROM categories WHERE industry_id = {} ORDER BY name", ind_id)
    } else {
        "SELECT id, name, description, created_at FROM categories WHERE industry_id IS NULL ORDER BY name".to_string()
    };

    let mut stmt = db
        .prepare(&sql)
        .map_err(|e| server_err(e.to_string()))?;

    let cats: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id":          row.get::<_, i64>(0)?,
                "name":        row.get::<_, String>(1)?,
                "description": row.get::<_, Option<String>>(2)?,
                "created_at":  row.get::<_, String>(3)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(cats)))
}

pub async fn create_category(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;
    let industry_id = get_user_industry_id(&db, &token)?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;
    let description = req["description"].as_str();

    db.execute(
        "INSERT INTO categories (name, description, industry_id) VALUES (?, ?, ?)",
        params![name.trim(), description, industry_id],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            bad_req("Category name already exists")
        } else {
            server_err(e.to_string())
        }
    })?;

    let id = db.last_insert_rowid();
    let cat = db
        .query_row(
            "SELECT id, name, description, created_at FROM categories WHERE id = ?",
            params![id],
            |row| {
                Ok(json!({
                    "id":          row.get::<_, i64>(0)?,
                    "name":        row.get::<_, String>(1)?,
                    "description": row.get::<_, Option<String>>(2)?,
                    "created_at":  row.get::<_, String>(3)?
                }))
            },
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(cat))
}

pub async fn update_category(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;
    let description = req["description"].as_str();

    db.execute(
        "UPDATE categories SET name = ?, description = ? WHERE id = ?",
        params![name.trim(), description, id],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let cat = db
        .query_row(
            "SELECT id, name, description, created_at FROM categories WHERE id = ?",
            params![id],
            |row| {
                Ok(json!({
                    "id":          row.get::<_, i64>(0)?,
                    "name":        row.get::<_, String>(1)?,
                    "description": row.get::<_, Option<String>>(2)?,
                    "created_at":  row.get::<_, String>(3)?
                }))
            },
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(cat))
}

// ── Products ──────────────────────────────────────────────────────────────────

const PRODUCT_SELECT: &str =
    "SELECT p.id, p.category_id, c.name as category_name, p.sku, p.name, p.description,
            p.unit, p.cost_price, p.selling_price, p.quantity, p.reorder_level,
            p.is_vat_exempt, p.is_active, p.industry_id, p.created_at, p.updated_at
     FROM products p
     LEFT JOIN categories c ON p.category_id = c.id";

fn row_to_product(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    Ok(json!({
        "id":            row.get::<_, i64>(0)?,
        "category_id":   row.get::<_, Option<i64>>(1)?,
        "category_name": row.get::<_, Option<String>>(2)?,
        "sku":           row.get::<_, String>(3)?,
        "name":          row.get::<_, String>(4)?,
        "description":   row.get::<_, Option<String>>(5)?,
        "unit":          row.get::<_, String>(6)?,
        "cost_price":    row.get::<_, f64>(7)?,
        "selling_price": row.get::<_, f64>(8)?,
        "quantity":      row.get::<_, i64>(9)?,
        "reorder_level": row.get::<_, i64>(10)?,
        "is_vat_exempt": row.get::<_, bool>(11)?,
        "is_active":     row.get::<_, bool>(12)?,
        "industry_id":   row.get::<_, Option<i64>>(13)?,
        "created_at":    row.get::<_, String>(14)?,
        "updated_at":    row.get::<_, String>(15)?
    }))
}

pub async fn get_products(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;
    let industry_id = get_user_industry_id(&db, &token)?;

    let filter = body.get("filter");

    // Parse filter fields (support camelCase from JS)
    let search = filter.and_then(|f| {
        f["search"].as_str().map(|s| format!("%{}%", s))
    });
    let category_id = filter.and_then(|f| {
        f["category_id"].as_i64().or_else(|| f["categoryId"].as_i64())
    });
    let low_stock_only = filter
        .and_then(|f| {
            f["low_stock_only"]
                .as_bool()
                .or_else(|| f["lowStockOnly"].as_bool())
        })
        .unwrap_or(false);
    let active_only = filter
        .and_then(|f| {
            f["active_only"]
                .as_bool()
                .or_else(|| f["activeOnly"].as_bool())
        })
        .unwrap_or(true);

    let sql = format!(
        "{} WHERE {}{}{}{}{}ORDER BY p.name",
        PRODUCT_SELECT,
        if active_only { "p.is_active = 1 " } else { "1=1 " },
        if let Some(ind_id) = industry_id { format!("AND p.industry_id = {} ", ind_id) } else { "AND p.industry_id IS NULL ".to_string() },
        if search.is_some() { "AND (p.name LIKE :search OR p.sku LIKE :search) " } else { "" },
        if category_id.is_some() { "AND p.category_id = :cat_id " } else { "" },
        if low_stock_only { "AND p.quantity <= p.reorder_level " } else { "" },
    );

    let mut stmt = db.prepare(&sql).map_err(|e| server_err(e.to_string()))?;

    let empty_search = String::new();
    let search_val = search.as_deref().unwrap_or(&empty_search);
    let cat_id_val = category_id.unwrap_or(0);

    let products: Vec<Value> = match (search.is_some(), category_id.is_some()) {
        (true, true) => stmt
            .query_map(
                rusqlite::named_params! {":search": search_val, ":cat_id": cat_id_val},
                row_to_product,
            )
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
        (true, false) => stmt
            .query_map(
                rusqlite::named_params! {":search": search_val},
                row_to_product,
            )
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
        (false, true) => stmt
            .query_map(
                rusqlite::named_params! {":cat_id": cat_id_val},
                row_to_product,
            )
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
        (false, false) => stmt
            .query_map([], row_to_product)
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
    };

    Ok(Json(json!(products)))
}

pub async fn get_product(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;

    let product = db
        .query_row(
            &format!("{} WHERE p.id = ?", PRODUCT_SELECT),
            params![id],
            row_to_product,
        )
        .map_err(|_| bad_req("Product not found"))?;

    Ok(Json(product))
}

pub async fn create_product(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;
    let industry_id = get_user_industry_id(&db, &token)?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let sku = req["sku"].as_str().ok_or_else(|| bad_req("Missing sku"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;

    if sku.trim().is_empty() { return Err(bad_req("SKU cannot be empty")); }
    if name.trim().is_empty() { return Err(bad_req("Product name cannot be empty")); }

    let category_id = req["category_id"].as_i64().or_else(|| req["categoryId"].as_i64());
    let description = req["description"].as_str();
    let unit = req["unit"].as_str().unwrap_or("piece");
    let cost_price = req["cost_price"].as_f64().or_else(|| req["costPrice"].as_f64()).unwrap_or(0.0);
    let selling_price = req["selling_price"].as_f64().or_else(|| req["sellingPrice"].as_f64()).unwrap_or(0.0);
    let quantity = req["quantity"].as_i64().unwrap_or(0);
    let reorder_level = req["reorder_level"].as_i64().or_else(|| req["reorderLevel"].as_i64()).unwrap_or(10);
    let is_vat_exempt = req["is_vat_exempt"].as_bool().or_else(|| req["isVatExempt"].as_bool()).unwrap_or(false);

    db.execute(
        "INSERT INTO products (category_id, sku, name, description, unit, cost_price, selling_price, quantity, reorder_level, is_vat_exempt, industry_id)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![category_id, sku.trim(), name.trim(), description, unit, cost_price, selling_price, quantity, reorder_level, is_vat_exempt, industry_id],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") { bad_req("SKU already exists") } else { server_err(e.to_string()) }
    })?;

    let id = db.last_insert_rowid();
    let product = db
        .query_row(&format!("{} WHERE p.id = ?", PRODUCT_SELECT), params![id], row_to_product)
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(product))
}

pub async fn update_product(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    macro_rules! upd_str {
        ($key:expr, $alt:expr, $col:literal) => {
            if let Some(v) = req[$key].as_str().or_else(|| req[$alt].as_str()) {
                db.execute(
                    concat!("UPDATE products SET ", $col, " = ?, updated_at = datetime('now') WHERE id = ?"),
                    params![v, id],
                ).map_err(|e| server_err(e.to_string()))?;
            }
        };
    }
    macro_rules! upd_f64 {
        ($key:expr, $alt:expr, $col:literal) => {
            if let Some(v) = req[$key].as_f64().or_else(|| req[$alt].as_f64()) {
                db.execute(
                    concat!("UPDATE products SET ", $col, " = ?, updated_at = datetime('now') WHERE id = ?"),
                    params![v, id],
                ).map_err(|e| server_err(e.to_string()))?;
            }
        };
    }
    macro_rules! upd_i64 {
        ($key:expr, $alt:expr, $col:literal) => {
            if let Some(v) = req[$key].as_i64().or_else(|| req[$alt].as_i64()) {
                db.execute(
                    concat!("UPDATE products SET ", $col, " = ?, updated_at = datetime('now') WHERE id = ?"),
                    params![v, id],
                ).map_err(|e| server_err(e.to_string()))?;
            }
        };
    }
    macro_rules! upd_bool {
        ($key:expr, $alt:expr, $col:literal) => {
            if let Some(v) = req[$key].as_bool().or_else(|| req[$alt].as_bool()) {
                db.execute(
                    concat!("UPDATE products SET ", $col, " = ?, updated_at = datetime('now') WHERE id = ?"),
                    params![v, id],
                ).map_err(|e| server_err(e.to_string()))?;
            }
        };
    }

    upd_i64!("category_id", "categoryId", "category_id");
    upd_str!("sku",         "sku",         "sku");
    upd_str!("name",        "name",        "name");
    upd_str!("description", "description", "description");
    upd_str!("unit",        "unit",        "unit");
    upd_f64!("cost_price",    "costPrice",    "cost_price");
    upd_f64!("selling_price", "sellingPrice", "selling_price");
    upd_i64!("reorder_level", "reorderLevel", "reorder_level");
    upd_bool!("is_vat_exempt", "isVatExempt", "is_vat_exempt");
    upd_bool!("is_active",     "isActive",    "is_active");

    let product = db
        .query_row(&format!("{} WHERE p.id = ?", PRODUCT_SELECT), params![id], row_to_product)
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(product))
}

pub async fn delete_product(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;
    db.execute("UPDATE products SET is_active = 0 WHERE id = ?", params![id])
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!(null)))
}

// ── Stock Adjustments ─────────────────────────────────────────────────────────

pub async fn adjust_stock(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let product_id = req["product_id"].as_i64()
        .or_else(|| req["productId"].as_i64())
        .ok_or_else(|| bad_req("Missing product_id"))?;

    let adjustment_type = req["adjustment_type"]
        .as_str()
        .or_else(|| req["adjustmentType"].as_str())
        .ok_or_else(|| bad_req("Missing adjustment_type"))?;

    let quantity = req["quantity"].as_i64().ok_or_else(|| bad_req("Missing quantity"))?;
    let reason = req["reason"].as_str();

    let (qty_before, product_name): (i64, String) = db
        .query_row(
            "SELECT quantity, name FROM products WHERE id = ?",
            params![product_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|_| bad_req("Product not found"))?;

    let (qty_change, qty_after) = match adjustment_type {
        "add" => (quantity, qty_before + quantity),
        "subtract" => {
            if qty_before < quantity {
                return Err(bad_req(format!(
                    "Insufficient stock. Current: {}, Requested: {}",
                    qty_before, quantity
                )));
            }
            (-quantity, qty_before - quantity)
        }
        "set" => (quantity - qty_before, quantity),
        _ => return Err(bad_req("Invalid adjustment type")),
    };

    db.execute(
        "UPDATE products SET quantity = ?, updated_at = datetime('now') WHERE id = ?",
        params![qty_after, product_id],
    )
    .map_err(|e| server_err(e.to_string()))?;

    db.execute(
        "INSERT INTO stock_adjustments (product_id, adjustment_type, quantity_before, quantity_change, quantity_after, reason, user_id)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![product_id, adjustment_type, qty_before, qty_change, qty_after, reason, session.user_id],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let adj_id = db.last_insert_rowid();
    let adj = db
        .query_row(
            "SELECT sa.id, sa.product_id, p.name, sa.adjustment_type, sa.quantity_before,
                    sa.quantity_change, sa.quantity_after, sa.reason, sa.user_id, u.full_name, sa.created_at
             FROM stock_adjustments sa
             JOIN products p ON sa.product_id = p.id
             JOIN users u ON sa.user_id = u.id
             WHERE sa.id = ?",
            params![adj_id],
            |row| {
                Ok(json!({
                    "id":              row.get::<_, i64>(0)?,
                    "product_id":      row.get::<_, i64>(1)?,
                    "product_name":    row.get::<_, String>(2)?,
                    "adjustment_type": row.get::<_, String>(3)?,
                    "quantity_before": row.get::<_, i64>(4)?,
                    "quantity_change": row.get::<_, i64>(5)?,
                    "quantity_after":  row.get::<_, i64>(6)?,
                    "reason":          row.get::<_, Option<String>>(7)?,
                    "user_id":         row.get::<_, i64>(8)?,
                    "adjusted_by":     row.get::<_, String>(9)?,
                    "created_at":      row.get::<_, String>(10)?
                }))
            },
        )
        .map_err(|e| server_err(e.to_string()))?;

    let _ = product_name; // used in error message above
    Ok(Json(adj))
}

pub async fn get_stock_adjustments(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    // Support camelCase productId and snake_case product_id
    let product_id = body["productId"].as_i64().or_else(|| body["product_id"].as_i64());

    let mut stmt = db
        .prepare(
            "SELECT sa.id, sa.product_id, p.name, sa.adjustment_type, sa.quantity_before,
                    sa.quantity_change, sa.quantity_after, sa.reason, sa.user_id, u.full_name, sa.created_at
             FROM stock_adjustments sa
             JOIN products p ON sa.product_id = p.id
             JOIN users u ON sa.user_id = u.id
             WHERE (? IS NULL OR sa.product_id = ?)
             ORDER BY sa.created_at DESC LIMIT 100",
        )
        .map_err(|e| server_err(e.to_string()))?;

    let adjustments: Vec<Value> = stmt
        .query_map(params![product_id, product_id], |row| {
            Ok(json!({
                "id":              row.get::<_, i64>(0)?,
                "product_id":      row.get::<_, i64>(1)?,
                "product_name":    row.get::<_, String>(2)?,
                "adjustment_type": row.get::<_, String>(3)?,
                "quantity_before": row.get::<_, i64>(4)?,
                "quantity_change": row.get::<_, i64>(5)?,
                "quantity_after":  row.get::<_, i64>(6)?,
                "reason":          row.get::<_, Option<String>>(7)?,
                "user_id":         row.get::<_, i64>(8)?,
                "adjusted_by":     row.get::<_, String>(9)?,
                "created_at":      row.get::<_, String>(10)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(adjustments)))
}
