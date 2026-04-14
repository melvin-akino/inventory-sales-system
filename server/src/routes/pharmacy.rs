use crate::{
    auth_guard::{extract_token, validate_session},
    routes::{bad_req, server_err},
    Db,
};
use axum::{extract::State, http::HeaderMap, Json};
use axum::http::StatusCode;
use rusqlite::params;
use serde_json::{json, Value};

// ============ Patients ============

pub async fn get_pharmacy_patients(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare(
            "SELECT id, name, phone, email, date_of_birth, gender, address, allergies, medical_conditions, 
                    insurance_provider, insurance_member_id, emergency_contact, emergency_contact_phone, is_active, created_at
             FROM pharmacy_patients WHERE is_active = 1 ORDER BY name"
        )
        .map_err(|e| server_err(e.to_string()))?;

    let patients = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "phone": row.get::<_, Option<String>>(2)?,
                "email": row.get::<_, Option<String>>(3)?,
                "date_of_birth": row.get::<_, Option<String>>(4)?,
                "gender": row.get::<_, Option<String>>(5)?,
                "address": row.get::<_, Option<String>>(6)?,
                "allergies": row.get::<_, Option<String>>(7)?,
                "medical_conditions": row.get::<_, Option<String>>(8)?,
                "insurance_provider": row.get::<_, Option<String>>(9)?,
                "insurance_member_id": row.get::<_, Option<String>>(10)?,
                "emergency_contact": row.get::<_, Option<String>>(11)?,
                "emergency_contact_phone": row.get::<_, Option<String>>(12)?,
                "is_active": row.get::<_, bool>(13)?,
                "created_at": row.get::<_, String>(14)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!({ "patients": patients })))
}

pub async fn create_pharmacy_patient(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request' field"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;

    db.execute(
        "INSERT INTO pharmacy_patients (name, phone, email, date_of_birth, gender, address, allergies, medical_conditions, 
                                       insurance_provider, insurance_member_id, emergency_contact, emergency_contact_phone)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            name,
            req["phone"].as_str(),
            req["email"].as_str(),
            req["date_of_birth"].as_str(),
            req["gender"].as_str(),
            req["address"].as_str(),
            req["allergies"].as_str(),
            req["medical_conditions"].as_str(),
            req["insurance_provider"].as_str(),
            req["insurance_member_id"].as_str(),
            req["emergency_contact"].as_str(),
            req["emergency_contact_phone"].as_str()
        ],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let id = db.last_insert_rowid();

    Ok(Json(json!({
        "id": id,
        "name": name,
        "message": "Patient created successfully"
    })))
}

pub async fn update_pharmacy_patient(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request' field"))?;
    let id = req["id"].as_i64().ok_or_else(|| bad_req("Missing patient id"))?;
    let name = req["name"].as_str().ok_or_else(|| bad_req("Missing name"))?;

    db.execute(
        "UPDATE pharmacy_patients SET name = ?, phone = ?, email = ?, date_of_birth = ?, gender = ?, address = ?, 
                allergies = ?, medical_conditions = ?, insurance_provider = ?, insurance_member_id = ?, 
                emergency_contact = ?, emergency_contact_phone = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
        params![
            name,
            req["phone"].as_str(),
            req["email"].as_str(),
            req["date_of_birth"].as_str(),
            req["gender"].as_str(),
            req["address"].as_str(),
            req["allergies"].as_str(),
            req["medical_conditions"].as_str(),
            req["insurance_provider"].as_str(),
            req["insurance_member_id"].as_str(),
            req["emergency_contact"].as_str(),
            req["emergency_contact_phone"].as_str(),
            id
        ],
    )
    .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!({
        "id": id,
        "name": name,
        "message": "Patient updated successfully"
    })))
}

// ============ Prescriptions ============

pub async fn get_prescriptions(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare(
            "SELECT id, prescription_number, patient_id, doctor_name, doctor_license, prescribing_date, 
                    expiry_date, instructions, refills_allowed, refills_used, is_controlled, status, notes, created_at
             FROM prescriptions ORDER BY prescribing_date DESC LIMIT 100"
        )
        .map_err(|e| server_err(e.to_string()))?;

    let prescriptions = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "prescription_number": row.get::<_, String>(1)?,
                "patient_id": row.get::<_, i64>(2)?,
                "doctor_name": row.get::<_, Option<String>>(3)?,
                "doctor_license": row.get::<_, Option<String>>(4)?,
                "prescribing_date": row.get::<_, String>(5)?,
                "expiry_date": row.get::<_, Option<String>>(6)?,
                "instructions": row.get::<_, Option<String>>(7)?,
                "refills_allowed": row.get::<_, i32>(8)?,
                "refills_used": row.get::<_, i32>(9)?,
                "is_controlled": row.get::<_, bool>(10)?,
                "status": row.get::<_, String>(11)?,
                "notes": row.get::<_, Option<String>>(12)?,
                "created_at": row.get::<_, String>(13)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!({ "prescriptions": prescriptions })))
}

pub async fn get_prescription_by_number(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let rx_number = body["prescription_number"]
        .as_str()
        .ok_or_else(|| bad_req("Missing prescription_number"))?;

    let prescription = db
        .query_row(
            "SELECT id, prescription_number, patient_id, doctor_name, doctor_license, prescribing_date, 
                    expiry_date, instructions, refills_allowed, refills_used, is_controlled, status, notes, created_at
             FROM prescriptions WHERE prescription_number = ?",
            params![rx_number],
            |row| {
                Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "prescription_number": row.get::<_, String>(1)?,
                    "patient_id": row.get::<_, i64>(2)?,
                    "doctor_name": row.get::<_, Option<String>>(3)?,
                    "doctor_license": row.get::<_, Option<String>>(4)?,
                    "prescribing_date": row.get::<_, String>(5)?,
                    "expiry_date": row.get::<_, Option<String>>(6)?,
                    "instructions": row.get::<_, Option<String>>(7)?,
                    "refills_allowed": row.get::<_, i32>(8)?,
                    "refills_used": row.get::<_, i32>(9)?,
                    "is_controlled": row.get::<_, bool>(10)?,
                    "status": row.get::<_, String>(11)?,
                    "notes": row.get::<_, Option<String>>(12)?,
                    "created_at": row.get::<_, String>(13)?
                }))
            },
        )
        .map_err(|_| (StatusCode::NOT_FOUND, "Prescription not found".to_string()))?;

    Ok(Json(json!({ "prescription": prescription })))
}

pub async fn create_prescription(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request' field"))?;
    let rx_number = req["prescription_number"]
        .as_str()
        .ok_or_else(|| bad_req("Missing prescription_number"))?;
    let patient_id = req["patient_id"].as_i64().ok_or_else(|| bad_req("Missing patient_id"))?;

    db.execute(
        "INSERT INTO prescriptions (prescription_number, patient_id, doctor_name, doctor_license, prescribing_date, 
                                   expiry_date, instructions, refills_allowed, is_controlled, status)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'active')",
        params![
            rx_number,
            patient_id,
            req["doctor_name"].as_str(),
            req["doctor_license"].as_str(),
            req["prescribing_date"].as_str().unwrap_or(""),
            req["expiry_date"].as_str(),
            req["instructions"].as_str(),
            req["refills_allowed"].as_i64().unwrap_or(0),
            req["is_controlled"].as_i64().unwrap_or(0) != 0
        ],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            bad_req("Prescription number already exists")
        } else {
            server_err(e.to_string())
        }
    })?;

    let id = db.last_insert_rowid();

    Ok(Json(json!({
        "id": id,
        "prescription_number": rx_number,
        "message": "Prescription created successfully"
    })))
}

// ============ Controlled Substances ============

pub async fn log_controlled_substance(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body.get("token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let req = body.get("request").ok_or_else(|| bad_req("Missing 'request' field"))?;

    db.execute(
        "INSERT INTO controlled_substance_log (product_id, product_name, transaction_type, quantity, user_id, patient_id, prescription_id, witness_name, reason)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            req["product_id"].as_i64().ok_or_else(|| bad_req("Missing product_id"))?,
            req["product_name"].as_str().ok_or_else(|| bad_req("Missing product_name"))?,
            req["transaction_type"].as_str().ok_or_else(|| bad_req("Missing transaction_type"))?,
            req["quantity"].as_i64().ok_or_else(|| bad_req("Missing quantity"))?,
            req["user_id"].as_i64().ok_or_else(|| bad_req("Missing user_id"))?,
            req["patient_id"].as_i64(),
            req["prescription_id"].as_i64(),
            req["witness_name"].as_str(),
            req["reason"].as_str()
        ],
    )
    .map_err(|e| server_err(e.to_string()))?;

    let id = db.last_insert_rowid();

    Ok(Json(json!({
        "id": id,
        "message": "Controlled substance transaction logged"
    })))
}

pub async fn get_controlled_substance_logs(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    let mut stmt = db
        .prepare(
            "SELECT id, product_id, product_name, transaction_type, quantity, user_id, patient_id, prescription_id, witness_name, reason, created_at
             FROM controlled_substance_log ORDER BY created_at DESC LIMIT 200"
        )
        .map_err(|e| server_err(e.to_string()))?;

    let logs = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "product_id": row.get::<_, i64>(1)?,
                "product_name": row.get::<_, String>(2)?,
                "transaction_type": row.get::<_, String>(3)?,
                "quantity": row.get::<_, i32>(4)?,
                "user_id": row.get::<_, i64>(5)?,
                "patient_id": row.get::<_, Option<i64>>(6)?,
                "prescription_id": row.get::<_, Option<i64>>(7)?,
                "witness_name": row.get::<_, Option<String>>(8)?,
                "reason": row.get::<_, Option<String>>(9)?,
                "created_at": row.get::<_, String>(10)?
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!({ "logs": logs })))
}

// ============ Expiry Alerts ============

pub async fn get_expiry_alerts(
    State(db): State<Db>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = extract_token(&headers).or_else(|_| {
        body["token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
    })?;

    let db = db.lock().map_err(|e| server_err(e.to_string()))?;
    validate_session(&db, &token)?;

    // Get medications expiring in next 30 days
    let mut stmt = db
        .prepare(
            "SELECT p.id, p.name, ppd.expiry_date, ppd.quantity, CAST((julianday(ppd.expiry_date) - julianday('now')) AS INTEGER) as days_until_expiry
             FROM products p
             LEFT JOIN pharmacy_product_details ppd ON p.id = ppd.product_id
             WHERE ppd.expiry_date IS NOT NULL
             AND ppd.expiry_date <= date('now', '+30 days')
             AND ppd.expiry_date > date('now')
             ORDER BY ppd.expiry_date ASC"
        )
        .map_err(|e| server_err(e.to_string()))?;

    let alerts = stmt
        .query_map([], |row| {
            Ok(json!({
                "product_id": row.get::<_, i64>(0)?,
                "product_name": row.get::<_, String>(1)?,
                "expiry_date": row.get::<_, String>(2)?,
                "quantity": row.get::<_, i32>(3)?,
                "days_until_expiry": row.get::<_, i32>(4)?,
                "alert_type": "expiry_near"
            }))
        })
        .map_err(|e| server_err(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| server_err(e.to_string()))?;

    Ok(Json(json!({ "alerts": alerts })))
}
