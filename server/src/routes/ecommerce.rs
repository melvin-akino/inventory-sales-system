use crate::{Db, auth_guard::{require_role, validate_session}, routes::{bad_req, server_err}};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};
use chrono::{Utc, Duration};
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────────────────────
// E-COMMERCE CUSTOMERS (Registration, Login, Profile)
// ────────────────────────────────────────────────────────────────────────────────

pub async fn ecommerce_register(
    State(db): State<Db>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let email = req["email"].as_str().ok_or_else(|| bad_req("Missing email"))?;
    let password = req["password"].as_str().ok_or_else(|| bad_req("Missing password"))?;
    let first_name = req["first_name"].as_str().or_else(|| req["firstName"].as_str()).ok_or_else(|| bad_req("Missing first_name"))?;
    let last_name = req["last_name"].as_str().or_else(|| req["lastName"].as_str()).ok_or_else(|| bad_req("Missing last_name"))?;
    let phone = req["phone"].as_str();

    if password.len() < 6 {
        return Err(bad_req("Password must be at least 6 characters"));
    }

    let password_hash = bcrypt::hash(password, 12).map_err(|e| server_err(e.to_string()))?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    db.execute(
        "INSERT INTO ecommerce_customers (email, password_hash, first_name, last_name, phone)
         VALUES (?, ?, ?, ?, ?)",
        params![email, password_hash, first_name, last_name, phone],
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            bad_req("Email already exists")
        } else {
            server_err(e.to_string())
        }
    })?;

    let customer_id = db.last_insert_rowid();
    let customer = get_customer_by_id(&db, customer_id)?;
    Ok(Json(customer))
}

pub async fn ecommerce_login(
    State(db): State<Db>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let email = req["email"].as_str().ok_or_else(|| bad_req("Missing email"))?;
    let password = req["password"].as_str().ok_or_else(|| bad_req("Missing password"))?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    let (customer_id, password_hash, first_name, last_name): (i64, String, String, String) = db
        .query_row(
            "SELECT id, password_hash, first_name, last_name FROM ecommerce_customers WHERE email = ? AND is_active = 1",
            params![email],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|_| bad_req("Invalid email or password"))?;

    let valid = bcrypt::verify(password, &password_hash).map_err(|e| server_err(e.to_string()))?;
    if !valid {
        return Err(bad_req("Invalid email or password"));
    }

    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::days(30);

    db.execute(
        "INSERT INTO ecommerce_sessions (token, customer_id, expires_at) VALUES (?, ?, ?)",
        params![&token, customer_id, expires_at.to_rfc3339()],
    ).map_err(|e| server_err(e.to_string()))?;

    let customer = get_customer_by_id(&db, customer_id)?;

    Ok(Json(json!({
        "token": token,
        "customer": customer
    })))
}

fn get_customer_by_id(db: &rusqlite::Connection, id: i64) -> Result<Value, (StatusCode, String)> {
    db.query_row(
        "SELECT id, email, first_name, last_name, phone, shipping_address, shipping_city, 
                shipping_state, shipping_zip, shipping_country, billing_address, billing_city, 
                billing_state, billing_zip, billing_country, is_active, created_at, updated_at
         FROM ecommerce_customers WHERE id = ?",
        params![id],
        |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "email": row.get::<_, String>(1)?,
                "first_name": row.get::<_, String>(2)?,
                "last_name": row.get::<_, String>(3)?,
                "phone": row.get::<_, Option<String>>(4)?,
                "shipping_address": row.get::<_, Option<String>>(5)?,
                "shipping_city": row.get::<_, Option<String>>(6)?,
                "shipping_state": row.get::<_, Option<String>>(7)?,
                "shipping_zip": row.get::<_, Option<String>>(8)?,
                "shipping_country": row.get::<_, Option<String>>(9)?,
                "billing_address": row.get::<_, Option<String>>(10)?,
                "billing_city": row.get::<_, Option<String>>(11)?,
                "billing_state": row.get::<_, Option<String>>(12)?,
                "billing_zip": row.get::<_, Option<String>>(13)?,
                "billing_country": row.get::<_, Option<String>>(14)?,
                "is_active": row.get::<_, bool>(15)?,
                "created_at": row.get::<_, String>(16)?,
                "updated_at": row.get::<_, String>(17)?
            }))
        }
    ).map_err(|_| bad_req("Customer not found"))
}

pub async fn ecommerce_get_customer(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = body["token"].as_str().ok_or_else(|| bad_req("Missing token"))?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    let customer_id: i64 = db
        .query_row(
            "SELECT customer_id FROM ecommerce_sessions WHERE token = ? AND expires_at > datetime('now')",
            params![token],
            |row| row.get(0),
        )
        .map_err(|_| bad_req("Invalid or expired session"))?;

    let customer = get_customer_by_id(&db, customer_id)?;
    Ok(Json(customer))
}

// ────────────────────────────────────────────────────────────────────────────────
// E-COMMERCE PRODUCTS (Catalog for customers)
// ────────────────────────────────────────────────────────────────────────────────

pub async fn ecommerce_get_products(
    State(db): State<Db>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let filter = body.get("filter");
    let search = filter.and_then(|f| f["search"].as_str()).map(|s| format!("%{}%", s));
    let category_id = filter.and_then(|f| f["category_id"].as_i64().or_else(|| f["categoryId"].as_i64()));

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    let mut sql = "SELECT p.id, p.category_id, c.name as category_name, p.sku, p.name, p.description, p.unit, p.selling_price, p.quantity, p.created_at
                   FROM products p LEFT JOIN categories c ON p.category_id = c.id
                   WHERE p.is_active = 1 AND p.quantity > 0".to_string();

    if search.is_some() {
        sql.push_str(" AND (p.name LIKE :search OR p.sku LIKE :search)");
    }
    if category_id.is_some() {
        sql.push_str(" AND p.category_id = :cat_id");
    }
    sql.push_str(" ORDER BY p.name");

    let mut stmt = db.prepare(&sql).map_err(|e| server_err(e.to_string()))?;

    let empty_search = String::new();
    let search_val = search.as_deref().unwrap_or(&empty_search);
    let cat_id_val = category_id.unwrap_or(0);

    let products: Vec<Value> = match (search.is_some(), category_id.is_some()) {
        (true, true) => stmt
            .query_map(
                rusqlite::named_params! {":search": search_val, ":cat_id": cat_id_val},
                |row| {
                    Ok(json!({
                        "id": row.get::<_, i64>(0)?,
                        "category_id": row.get::<_, Option<i64>>(1)?,
                        "category_name": row.get::<_, Option<String>>(2)?,
                        "sku": row.get::<_, String>(3)?,
                        "name": row.get::<_, String>(4)?,
                        "description": row.get::<_, Option<String>>(5)?,
                        "unit": row.get::<_, String>(6)?,
                        "price": row.get::<_, f64>(7)?,
                        "stock": row.get::<_, i64>(8)?,
                        "created_at": row.get::<_, String>(9)?
                    }))
                }
            )
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
        (true, false) => stmt
            .query_map(
                rusqlite::named_params! {":search": search_val},
                |row| {
                    Ok(json!({
                        "id": row.get::<_, i64>(0)?,
                        "category_id": row.get::<_, Option<i64>>(1)?,
                        "category_name": row.get::<_, Option<String>>(2)?,
                        "sku": row.get::<_, String>(3)?,
                        "name": row.get::<_, String>(4)?,
                        "description": row.get::<_, Option<String>>(5)?,
                        "unit": row.get::<_, String>(6)?,
                        "price": row.get::<_, f64>(7)?,
                        "stock": row.get::<_, i64>(8)?,
                        "created_at": row.get::<_, String>(9)?
                    }))
                }
            )
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
        (false, true) => stmt
            .query_map(
                rusqlite::named_params! {":cat_id": cat_id_val},
                |row| {
                    Ok(json!({
                        "id": row.get::<_, i64>(0)?,
                        "category_id": row.get::<_, Option<i64>>(1)?,
                        "category_name": row.get::<_, Option<String>>(2)?,
                        "sku": row.get::<_, String>(3)?,
                        "name": row.get::<_, String>(4)?,
                        "description": row.get::<_, Option<String>>(5)?,
                        "unit": row.get::<_, String>(6)?,
                        "price": row.get::<_, f64>(7)?,
                        "stock": row.get::<_, i64>(8)?,
                        "created_at": row.get::<_, String>(9)?
                    }))
                }
            )
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
        (false, false) => stmt
            .query_map([], |row| {
                Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "category_id": row.get::<_, Option<i64>>(1)?,
                    "category_name": row.get::<_, Option<String>>(2)?,
                    "sku": row.get::<_, String>(3)?,
                    "name": row.get::<_, String>(4)?,
                    "description": row.get::<_, Option<String>>(5)?,
                    "unit": row.get::<_, String>(6)?,
                    "price": row.get::<_, f64>(7)?,
                    "stock": row.get::<_, i64>(8)?,
                    "created_at": row.get::<_, String>(9)?
                }))
            })
            .map_err(|e| server_err(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect(),
    };

    Ok(Json(json!(products)))
}

// ────────────────────────────────────────────────────────────────────────────────
// ORDERS & CHECKOUT
// ────────────────────────────────────────────────────────────────────────────────

pub async fn checkout(
    State(db): State<Db>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = body["token"].as_str().ok_or_else(|| bad_req("Missing token"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let items_json = req["items"].as_array().ok_or_else(|| bad_req("Missing items"))?;
    if items_json.is_empty() {
        return Err(bad_req("Cart cannot be empty"));
    }

    let shipping_address = req["shipping_address"].as_str().ok_or_else(|| bad_req("Missing shipping_address"))?;
    let payment_method = req["payment_method"].as_str().or_else(|| req["paymentMethod"].as_str()).unwrap_or("card");

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    let customer_id: i64 = db
        .query_row(
            "SELECT customer_id FROM ecommerce_sessions WHERE token = ? AND expires_at > datetime('now')",
            params![token],
            |row| row.get(0),
        )
        .map_err(|_| bad_req("Invalid or expired session"))?;

    // Validate stock & compute totals
    let mut order_items: Vec<(i64, String, i64, f64, f64)> = Vec::new();
    let mut subtotal = 0.0f64;

    for item in items_json {
        let product_id = item["product_id"].as_i64().ok_or_else(|| bad_req("Missing product_id"))?;
        let quantity = item["quantity"].as_i64().ok_or_else(|| bad_req("Missing quantity"))?;

        if quantity <= 0 {
            return Err(bad_req("Quantity must be > 0"));
        }

        let (stock, name, price): (i64, String, f64) = db
            .query_row(
                "SELECT quantity, name, selling_price FROM products WHERE id = ? AND is_active = 1",
                params![product_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .map_err(|_| bad_req(format!("Product {} not found", product_id)))?;

        if stock < quantity {
            return Err(bad_req(format!("{}: only {} available", name, stock)));
        }

        let total = price * quantity as f64;
        subtotal += total;
        order_items.push((product_id, name, quantity, price, total));
    }

    let shipping_cost = 0.0;
    let tax_rate = 0.12;
    let tax_amount = subtotal * tax_rate;
    let discount_amount = req["discount_amount"].as_f64().or_else(|| req["discountAmount"].as_f64()).unwrap_or(0.0);
    let total_amount = subtotal + tax_amount + shipping_cost - discount_amount;

    // Create order
    let order_number = format!("ORD-{}", Uuid::new_v4().to_string()[..8].to_uppercase());
    db.execute(
        "INSERT INTO orders (order_number, customer_id, payment_method, subtotal, shipping_cost, tax_amount, discount_amount, total_amount, shipping_address, order_status, payment_status)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending', 'unpaid')",
        params![&order_number, customer_id, payment_method, subtotal, shipping_cost, tax_amount, discount_amount, total_amount, shipping_address],
    ).map_err(|e| server_err(e.to_string()))?;

    let order_id = db.last_insert_rowid();

    // Add order items & deduct inventory
    for (product_id, product_name, quantity, unit_price, total) in order_items {
        db.execute(
            "INSERT INTO order_items (order_id, product_id, product_name, quantity, unit_price, total_price)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![order_id, product_id, product_name, quantity, unit_price, total],
        ).map_err(|e| server_err(e.to_string()))?;

        db.execute(
            "UPDATE products SET quantity = quantity - ? WHERE id = ?",
            params![quantity, product_id],
        ).map_err(|e| server_err(e.to_string()))?;
    }

    let order = get_order_by_id(&db, order_id)?;
    Ok(Json(order))
}

fn get_order_by_id(db: &rusqlite::Connection, id: i64) -> Result<Value, (StatusCode, String)> {
    let order_row = db.query_row(
        "SELECT id, order_number, customer_id, order_status, payment_status, payment_method, 
                subtotal, shipping_cost, tax_amount, discount_amount, total_amount, shipping_address, created_at
         FROM orders WHERE id = ?",
        params![id],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, f64>(6)?,
                row.get::<_, f64>(7)?,
                row.get::<_, f64>(8)?,
                row.get::<_, f64>(9)?,
                row.get::<_, f64>(10)?,
                row.get::<_, Option<String>>(11)?,
                row.get::<_, String>(12)?,
            ))
        }
    ).map_err(|_| bad_req("Order not found"))?;

    let mut stmt = db.prepare(
        "SELECT id, product_id, product_name, quantity, unit_price, total_price FROM order_items WHERE order_id = ?"
    ).map_err(|e| server_err(e.to_string()))?;

    let items: Vec<Value> = stmt.query_map(params![order_row.0], |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "product_id": row.get::<_, i64>(1)?,
            "product_name": row.get::<_, String>(2)?,
            "quantity": row.get::<_, i64>(3)?,
            "unit_price": row.get::<_, f64>(4)?,
            "total_price": row.get::<_, f64>(5)?
        }))
    }).map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({
        "id": order_row.0,
        "order_number": order_row.1,
        "customer_id": order_row.2,
        "order_status": order_row.3,
        "payment_status": order_row.4,
        "payment_method": order_row.5,
        "subtotal": order_row.6,
        "shipping_cost": order_row.7,
        "tax_amount": order_row.8,
        "discount_amount": order_row.9,
        "total_amount": order_row.10,
        "shipping_address": order_row.11,
        "items": items,
        "created_at": order_row.12
    }))
}

// ────────────────────────────────────────────────────────────────────────────────
// MOCK PAYMENT (Process payment with mock gateway)
// ────────────────────────────────────────────────────────────────────────────────

pub async fn mock_payment(
    State(db): State<Db>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = body["token"].as_str().ok_or_else(|| bad_req("Missing token"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let order_id = req["order_id"].as_i64().or_else(|| req["orderId"].as_i64()).ok_or_else(|| bad_req("Missing order_id"))?;
    let payment_method = req["payment_method"].as_str().or_else(|| req["paymentMethod"].as_str()).unwrap_or("card");
    
    // Simulate 95% success rate
    let success = rand::random::<f32>() < 0.95;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;

    // Verify customer owns the order
    let (customer_id, total_amount): (i64, f64) = db
        .query_row(
            "SELECT customer_id, total_amount FROM orders WHERE id = ?",
            params![order_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| bad_req("Order not found"))?;

    let order_customer: i64 = db
        .query_row(
            "SELECT customer_id FROM ecommerce_sessions WHERE token = ? AND expires_at > datetime('now')",
            params![token],
            |row| row.get(0),
        )
        .map_err(|_| bad_req("Invalid session"))?;

    if customer_id != order_customer {
        return Err(bad_req("Unauthorized"));
    }

    // Create payment record
    let transaction_id = if success {
        format!("MOCK-{}", Uuid::new_v4().to_string()[..12].to_uppercase())
    } else {
        String::new()
    };

    let payment_status = if success { "completed" } else { "failed" };
    let error_message = if !success { Some("Simulated payment failure") } else { None };

    db.execute(
        "INSERT INTO payments (order_id, amount, payment_method, payment_status, payment_gateway, transaction_id, error_message)
         VALUES (?, ?, ?, ?, 'mock', ?, ?)",
        params![order_id, total_amount, payment_method, payment_status, &transaction_id, error_message],
    ).map_err(|e| server_err(e.to_string()))?;

    // Update order status if payment successful
    if success {
        db.execute(
            "UPDATE orders SET payment_status = 'paid', order_status = 'processing' WHERE id = ?",
            params![order_id],
        ).map_err(|e| server_err(e.to_string()))?;
    } else {
        db.execute(
            "UPDATE orders SET payment_status = 'failed' WHERE id = ?",
            params![order_id],
        ).map_err(|e| server_err(e.to_string()))?;
    }

    let order = get_order_by_id(&db, order_id)?;

    Ok(Json(json!({
        "success": success,
        "payment_status": payment_status,
        "transaction_id": transaction_id,
        "order": order
    })))
}

// ────────────────────────────────────────────────────────────────────────────────
// ADMIN: GET ALL ORDERS (for order management page)
// ────────────────────────────────────────────────────────────────────────────────

pub async fn admin_get_orders(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = body["token"].as_str().ok_or_else(|| bad_req("Missing token for auth"))?;
    
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    
    // Validate admin session (from regular admin system, not ecommerce)
    let _session = crate::auth_guard::validate_session(&db, token)?;
    crate::auth_guard::require_role(&_session, &["super_admin", "admin", "manager"])?;

    let mut stmt = db.prepare(
        "SELECT o.id, o.order_number, o.customer_id, ec.email, ec.first_name, ec.last_name, 
                o.order_status, o.payment_status, o.payment_method, o.total_amount, o.created_at
         FROM orders o
         LEFT JOIN ecommerce_customers ec ON o.customer_id = ec.id
         ORDER BY o.created_at DESC LIMIT 200"
    ).map_err(|e| server_err(e.to_string()))?;

    let orders: Vec<Value> = stmt.query_map([], |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "order_number": row.get::<_, String>(1)?,
            "customer_id": row.get::<_, i64>(2)?,
            "customer_email": row.get::<_, Option<String>>(3)?,
            "customer_name": format!("{} {}", row.get::<_, Option<String>>(4)?.unwrap_or_default(), row.get::<_, Option<String>>(5)?.unwrap_or_default()),
            "order_status": row.get::<_, String>(6)?,
            "payment_status": row.get::<_, String>(7)?,
            "payment_method": row.get::<_, String>(8)?,
            "total_amount": row.get::<_, f64>(9)?,
            "created_at": row.get::<_, String>(10)?
        }))
    }).map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(orders)))
}

// ────────────────────────────────────────────────────────────────────────────────
// ADMIN: UPDATE ORDER STATUS
// ────────────────────────────────────────────────────────────────────────────────

pub async fn admin_update_order_status(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = body["token"].as_str().ok_or_else(|| bad_req("Missing token for auth"))?;
    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;

    let order_id = req["order_id"].as_i64().or_else(|| req["orderId"].as_i64()).ok_or_else(|| bad_req("Missing order_id"))?;
    let new_status = req["status"].as_str().ok_or_else(|| bad_req("Missing status"))?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    
    let _session = crate::auth_guard::validate_session(&db, token)?;
    crate::auth_guard::require_role(&_session, &["super_admin", "admin", "manager"])?;

    // Validate status
    if !["pending", "processing", "shipped", "delivered", "cancelled", "refunded"].contains(&new_status) {
        return Err(bad_req("Invalid status"));
    }

    db.execute(
        "UPDATE orders SET order_status = ?, updated_at = datetime('now') WHERE id = ?",
        params![new_status, order_id],
    ).map_err(|e| server_err(e.to_string()))?;

    let order = get_order_by_id(&db, order_id)?;
    Ok(Json(order))
}
