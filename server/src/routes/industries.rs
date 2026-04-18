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

/// Get all available industries
pub async fn get_industries(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare(
            "SELECT id, code, name, description, color, is_active, created_at
             FROM industries
             WHERE is_active = 1
             ORDER BY name"
        )
        .map_err(|e| server_err(e.to_string()))?;

    let industries: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id":          row.get::<_, i64>(0)?,
                "code":        row.get::<_, String>(1)?,
                "name":        row.get::<_, String>(2)?,
                "description": row.get::<_, Option<String>>(3)?,
                "color":       row.get::<_, Option<String>>(4)?,
                "is_active":   row.get::<_, bool>(5)?,
                "created_at":  row.get::<_, String>(6)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(Json(json!(industries)))
}

/// Get industry by ID with its attributes
pub async fn get_industry(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let industry_id = body["id"].as_i64().ok_or_else(|| bad_req("Missing id"))?;

    let mut industry = db
        .query_row(
            "SELECT id, code, name, description, color, is_active, created_at
             FROM industries
             WHERE id = ?",
            params![industry_id],
            |row| {
                Ok(json!({
                    "id":          row.get::<_, i64>(0)?,
                    "code":        row.get::<_, String>(1)?,
                    "name":        row.get::<_, String>(2)?,
                    "description": row.get::<_, Option<String>>(3)?,
                    "color":       row.get::<_, Option<String>>(4)?,
                    "is_active":   row.get::<_, bool>(5)?,
                    "created_at":  row.get::<_, String>(6)?
                }))
            },
        )
        .map_err(|_| bad_req("Industry not found"))?;

    // Get industry-specific product attributes
    let mut stmt = db
        .prepare(
            "SELECT id, attribute_name, attribute_label, attribute_type, is_required, display_order
             FROM product_attributes
             WHERE industry_id = ?
             ORDER BY display_order, attribute_label"
        )
        .map_err(|e| server_err(e.to_string()))?;

    let attributes: Vec<Value> = stmt
        .query_map(params![industry_id], |row| {
            Ok(json!({
                "id":              row.get::<_, i64>(0)?,
                "attribute_name":  row.get::<_, String>(1)?,
                "attribute_label": row.get::<_, String>(2)?,
                "attribute_type":  row.get::<_, String>(3)?,
                "is_required":     row.get::<_, bool>(4)?,
                "display_order":   row.get::<_, i64>(5)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();

    industry["attributes"] = json!(attributes);
    Ok(Json(industry))
}

/// Add a new attribute to an industry
pub async fn add_industry_attribute(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let industry_id = req["industry_id"].as_i64().ok_or_else(|| bad_req("Missing industry_id"))?;
    let attribute_name = req["attribute_name"].as_str().ok_or_else(|| bad_req("Missing attribute_name"))?;
    let attribute_label = req["attribute_label"].as_str().ok_or_else(|| bad_req("Missing attribute_label"))?;
    let attribute_type = req["attribute_type"].as_str().unwrap_or("text");
    let is_required = req["is_required"].as_bool().unwrap_or(false);
    let display_order = req["display_order"].as_i64().unwrap_or(0);

    // Validate attribute type
    match attribute_type {
        "text" | "number" | "checkbox" | "select" | "date" | "textarea" => {},
        _ => return Err(bad_req("Invalid attribute_type")),
    }

    db.execute(
        "INSERT INTO product_attributes (industry_id, attribute_name, attribute_label, attribute_type, is_required, display_order)
         VALUES (?, ?, ?, ?, ?, ?)",
        params![industry_id, attribute_name, attribute_label, attribute_type, is_required, display_order],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            bad_req("Attribute already exists for this industry")
        } else {
            server_err(e.to_string())
        }
    })?;

    let id = db.last_insert_rowid();
    let attr = db
        .query_row(
            "SELECT id, attribute_name, attribute_label, attribute_type, is_required, display_order
             FROM product_attributes
             WHERE id = ?",
            params![id],
            |row| {
                Ok(json!({
                    "id":              row.get::<_, i64>(0)?,
                    "attribute_name":  row.get::<_, String>(1)?,
                    "attribute_label": row.get::<_, String>(2)?,
                    "attribute_type":  row.get::<_, String>(3)?,
                    "is_required":     row.get::<_, bool>(4)?,
                    "display_order":   row.get::<_, i64>(5)?
                }))
            },
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(attr))
}

/// Assign a user to an industry
pub async fn assign_user_to_industry(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = get_token(&headers, &body)?;
    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    let session = validate_session(&db, &token)?;
    require_role(&session, &["super_admin", "admin"])?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request'"))?;
    let user_id = req["user_id"].as_i64().ok_or_else(|| bad_req("Missing user_id"))?;
    let industry_id = req["industry_id"].as_i64().ok_or_else(|| bad_req("Missing industry_id"))?;

    db.execute(
        "UPDATE users SET industry_id = ? WHERE id = ?",
        params![industry_id, user_id],
    )
    .map_err(|e| server_err(e.to_string()))?;

    // Fetch updated user
    let user = db
        .query_row(
            "SELECT u.id, u.username, u.full_name, u.email, u.role, u.is_active, u.industry_id,
                    i.code as industry_code, i.name as industry_name, u.created_at, u.updated_at
             FROM users u
             LEFT JOIN industries i ON u.industry_id = i.id
             WHERE u.id = ?",
            params![user_id],
            |row| {
                let mut user_obj = json!({
                    "id":         row.get::<_, i64>(0)?,
                    "username":   row.get::<_, String>(1)?,
                    "full_name":  row.get::<_, String>(2)?,
                    "email":      row.get::<_, Option<String>>(3)?,
                    "role":       row.get::<_, String>(4)?,
                    "is_active":  row.get::<_, bool>(5)?,
                    "created_at": row.get::<_, String>(9)?,
                    "updated_at": row.get::<_, String>(10)?
                });

                if let Ok(Some(ind_id)) = row.get::<_, Option<i64>>(6) {
                    if let (Ok(Some(ind_code)), Ok(Some(ind_name))) = (
                        row.get::<_, Option<String>>(7),
                        row.get::<_, Option<String>>(8)
                    ) {
                        user_obj["industry"] = json!({
                            "id": ind_id,
                            "code": ind_code,
                            "name": ind_name
                        });
                    }
                }

                Ok(user_obj)
            },
        )
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(user))
}
