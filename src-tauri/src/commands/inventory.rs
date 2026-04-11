use crate::commands::auth::{require_role, validate_session};
use crate::models::product::{
    Category, CreateCategoryRequest, CreateProductRequest, Product, ProductFilter,
    StockAdjustment, StockAdjustmentRequest, UpdateProductRequest,
};
use crate::AppState;
use rusqlite::params;
use tauri::State;

// ── Categories ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_categories(
    token: String,
    state: State<'_, AppState>,
) -> Result<Vec<Category>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare("SELECT id, name, description, created_at FROM categories ORDER BY name")
        .map_err(|e| e.to_string())?;

    let cats = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(cats)
}

#[tauri::command]
pub async fn create_category(
    token: String,
    request: CreateCategoryRequest,
    state: State<'_, AppState>,
) -> Result<Category, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    db.execute(
        "INSERT INTO categories (name, description) VALUES (?, ?)",
        params![request.name.trim(), request.description],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            "Category name already exists".to_string()
        } else {
            e.to_string()
        }
    })?;

    let id = db.last_insert_rowid();
    db.query_row(
        "SELECT id, name, description, created_at FROM categories WHERE id = ?",
        params![id],
        |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_category(
    token: String,
    id: i64,
    request: CreateCategoryRequest,
    state: State<'_, AppState>,
) -> Result<Category, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    db.execute(
        "UPDATE categories SET name = ?, description = ? WHERE id = ?",
        params![request.name.trim(), request.description, id],
    )
    .map_err(|e| e.to_string())?;

    db.query_row(
        "SELECT id, name, description, created_at FROM categories WHERE id = ?",
        params![id],
        |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

// ── Products ──────────────────────────────────────────────────────────────────

fn row_to_product(row: &rusqlite::Row) -> rusqlite::Result<Product> {
    Ok(Product {
        id: row.get(0)?,
        category_id: row.get(1)?,
        category_name: row.get(2)?,
        sku: row.get(3)?,
        name: row.get(4)?,
        description: row.get(5)?,
        unit: row.get(6)?,
        cost_price: row.get(7)?,
        selling_price: row.get(8)?,
        quantity: row.get(9)?,
        reorder_level: row.get(10)?,
        is_vat_exempt: row.get(11)?,
        is_active: row.get(12)?,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })
}

const PRODUCT_SELECT: &str =
    "SELECT p.id, p.category_id, c.name as category_name, p.sku, p.name, p.description,
            p.unit, p.cost_price, p.selling_price, p.quantity, p.reorder_level,
            p.is_vat_exempt, p.is_active, p.created_at, p.updated_at
     FROM products p
     LEFT JOIN categories c ON p.category_id = c.id";

#[tauri::command]
pub async fn get_products(
    token: String,
    filter: Option<ProductFilter>,
    state: State<'_, AppState>,
) -> Result<Vec<Product>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    let mut conditions = vec!["1=1"];
    let mut search_val = String::new();
    let mut cat_id_val = 0i64;

    let f = filter.unwrap_or(ProductFilter {
        search: None,
        category_id: None,
        low_stock_only: None,
        active_only: Some(true),
    });

    let active_only = f.active_only.unwrap_or(true);
    let low_stock_only = f.low_stock_only.unwrap_or(false);

    let sql = format!(
        "{} WHERE {}{}{}{}
         ORDER BY p.name",
        PRODUCT_SELECT,
        if active_only { "p.is_active = 1 " } else { "1=1 " },
        if f.search.is_some() {
            search_val = format!("%{}%", f.search.as_ref().unwrap());
            "AND (p.name LIKE :search OR p.sku LIKE :search) "
        } else {
            ""
        },
        if f.category_id.is_some() {
            cat_id_val = f.category_id.unwrap();
            "AND p.category_id = :cat_id "
        } else {
            ""
        },
        if low_stock_only {
            "AND p.quantity <= p.reorder_level "
        } else {
            ""
        },
    );

    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;

    let products: Vec<Product> = match (f.search.is_some(), f.category_id.is_some()) {
        (true, true) => stmt
            .query_map(
                rusqlite::named_params! {":search": search_val, ":cat_id": cat_id_val},
                row_to_product,
            )
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect(),
        (true, false) => stmt
            .query_map(
                rusqlite::named_params! {":search": search_val},
                row_to_product,
            )
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect(),
        (false, true) => stmt
            .query_map(
                rusqlite::named_params! {":cat_id": cat_id_val},
                row_to_product,
            )
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect(),
        (false, false) => stmt
            .query_map([], row_to_product)
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect(),
    };

    Ok(products)
}

#[tauri::command]
pub async fn get_product(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<Product, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    validate_session(&db, &token)?;

    db.query_row(
        &format!("{} WHERE p.id = ?", PRODUCT_SELECT),
        params![id],
        row_to_product,
    )
    .map_err(|_| "Product not found".to_string())
}

#[tauri::command]
pub async fn create_product(
    token: String,
    request: CreateProductRequest,
    state: State<'_, AppState>,
) -> Result<Product, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    if request.sku.trim().is_empty() {
        return Err("SKU cannot be empty".to_string());
    }
    if request.name.trim().is_empty() {
        return Err("Product name cannot be empty".to_string());
    }

    db.execute(
        "INSERT INTO products (category_id, sku, name, description, unit, cost_price, selling_price, quantity, reorder_level, is_vat_exempt)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            request.category_id,
            request.sku.trim(),
            request.name.trim(),
            request.description,
            request.unit,
            request.cost_price,
            request.selling_price,
            request.quantity,
            request.reorder_level,
            request.is_vat_exempt
        ],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            "SKU already exists".to_string()
        } else {
            e.to_string()
        }
    })?;

    let id = db.last_insert_rowid();
    db.query_row(
        &format!("{} WHERE p.id = ?", PRODUCT_SELECT),
        params![id],
        row_to_product,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_product(
    token: String,
    id: i64,
    request: UpdateProductRequest,
    state: State<'_, AppState>,
) -> Result<Product, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    macro_rules! update_field {
        ($field:expr, $col:expr) => {
            if let Some(val) = $field {
                db.execute(
                    &format!(
                        "UPDATE products SET {} = ?, updated_at = datetime('now') WHERE id = ?",
                        $col
                    ),
                    params![val, id],
                )
                .map_err(|e| e.to_string())?;
            }
        };
    }

    update_field!(request.category_id, "category_id");
    update_field!(request.sku.as_deref(), "sku");
    update_field!(request.name.as_deref(), "name");
    update_field!(request.description.as_deref(), "description");
    update_field!(request.unit.as_deref(), "unit");
    update_field!(request.cost_price, "cost_price");
    update_field!(request.selling_price, "selling_price");
    update_field!(request.reorder_level, "reorder_level");
    update_field!(request.is_vat_exempt, "is_vat_exempt");
    update_field!(request.is_active, "is_active");

    db.query_row(
        &format!("{} WHERE p.id = ?", PRODUCT_SELECT),
        params![id],
        row_to_product,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_product(
    token: String,
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    db.execute(
        "UPDATE products SET is_active = 0 WHERE id = ?",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn adjust_stock(
    token: String,
    request: StockAdjustmentRequest,
    state: State<'_, AppState>,
) -> Result<StockAdjustment, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let (qty_before, product_name): (i64, String) = db
        .query_row(
            "SELECT quantity, name FROM products WHERE id = ?",
            params![request.product_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|_| "Product not found".to_string())?;

    let (qty_change, qty_after) = match request.adjustment_type.as_str() {
        "add" => (request.quantity, qty_before + request.quantity),
        "subtract" => {
            if qty_before < request.quantity {
                return Err(format!(
                    "Insufficient stock. Current: {}, Requested: {}",
                    qty_before, request.quantity
                ));
            }
            (-request.quantity, qty_before - request.quantity)
        }
        "set" => (request.quantity - qty_before, request.quantity),
        _ => return Err("Invalid adjustment type".to_string()),
    };

    db.execute(
        "UPDATE products SET quantity = ?, updated_at = datetime('now') WHERE id = ?",
        params![qty_after, request.product_id],
    )
    .map_err(|e| e.to_string())?;

    db.execute(
        "INSERT INTO stock_adjustments (product_id, adjustment_type, quantity_before, quantity_change, quantity_after, reason, user_id)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            request.product_id,
            request.adjustment_type,
            qty_before,
            qty_change,
            qty_after,
            request.reason,
            session.user_id
        ],
    )
    .map_err(|e| e.to_string())?;

    let adj_id = db.last_insert_rowid();
    db.query_row(
        "SELECT sa.id, sa.product_id, p.name, sa.adjustment_type, sa.quantity_before,
                sa.quantity_change, sa.quantity_after, sa.reason, sa.user_id, u.full_name, sa.created_at
         FROM stock_adjustments sa
         JOIN products p ON sa.product_id = p.id
         JOIN users u ON sa.user_id = u.id
         WHERE sa.id = ?",
        params![adj_id],
        |row| {
            Ok(StockAdjustment {
                id: row.get(0)?,
                product_id: row.get(1)?,
                product_name: row.get(2)?,
                adjustment_type: row.get(3)?,
                quantity_before: row.get(4)?,
                quantity_change: row.get(5)?,
                quantity_after: row.get(6)?,
                reason: row.get(7)?,
                user_id: row.get(8)?,
                adjusted_by: row.get(9)?,
                created_at: row.get(10)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_stock_adjustments(
    token: String,
    product_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<StockAdjustment>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin", "manager"])?;

    let sql = "SELECT sa.id, sa.product_id, p.name, sa.adjustment_type, sa.quantity_before,
                      sa.quantity_change, sa.quantity_after, sa.reason, sa.user_id, u.full_name, sa.created_at
               FROM stock_adjustments sa
               JOIN products p ON sa.product_id = p.id
               JOIN users u ON sa.user_id = u.id
               WHERE (? IS NULL OR sa.product_id = ?)
               ORDER BY sa.created_at DESC LIMIT 100";

    let mut stmt = db.prepare(sql).map_err(|e| e.to_string())?;
    let adjustments = stmt
        .query_map(params![product_id, product_id], |row| {
            Ok(StockAdjustment {
                id: row.get(0)?,
                product_id: row.get(1)?,
                product_name: row.get(2)?,
                adjustment_type: row.get(3)?,
                quantity_before: row.get(4)?,
                quantity_change: row.get(5)?,
                quantity_after: row.get(6)?,
                reason: row.get(7)?,
                user_id: row.get(8)?,
                adjusted_by: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(adjustments)
}
