use rusqlite::{Connection, Result};

pub fn initialize_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    run_migrations(&conn)?;
    Ok(conn)
}

fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    let current_version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    if current_version < 1 {
        migration_v1(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (1)", [])?;
    }

    // Add migration v2 to fix the admin password hash
    if current_version < 2 {
        migration_v2(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (2)", [])?;
    }

    // Add migration v3 to seed products
    if current_version < 3 {
        migration_v3(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (3)", [])?;
    }

    // Add migration v4 for pharmacy system
    if current_version < 4 {
        migration_v4(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (4)", [])?;
    }

    Ok(())
}

fn migration_v1(conn: &Connection) -> Result<()> {
    // Generate the password hash for "Admin@123"
    let password_hash = bcrypt::hash("Admin@123", 12)
        .unwrap_or_else(|_| "$2b$12$N9qo8uLOickgx2ZMRZoMyeIjZAgcg7b3XeKeUxWdeS86E36P4/aSi".to_string());

    conn.execute_batch(
        "
        -- Users
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            full_name TEXT NOT NULL,
            email TEXT,
            role TEXT NOT NULL DEFAULT 'cashier'
                CHECK(role IN ('super_admin','admin','manager','cashier','viewer')),
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Sessions
        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            expires_at DATETIME NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- Categories
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Products
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category_id INTEGER,
            sku TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            unit TEXT NOT NULL DEFAULT 'piece',
            cost_price REAL NOT NULL DEFAULT 0,
            selling_price REAL NOT NULL DEFAULT 0,
            quantity INTEGER NOT NULL DEFAULT 0,
            reorder_level INTEGER NOT NULL DEFAULT 10,
            is_vat_exempt INTEGER NOT NULL DEFAULT 0,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );

        -- Suppliers
        CREATE TABLE IF NOT EXISTS suppliers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            contact_person TEXT,
            phone TEXT,
            email TEXT,
            address TEXT,
            tin_number TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Customers
        CREATE TABLE IF NOT EXISTS customers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT,
            email TEXT,
            address TEXT,
            tin_number TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Sales
        CREATE TABLE IF NOT EXISTS sales (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sale_number TEXT UNIQUE NOT NULL,
            customer_id INTEGER,
            user_id INTEGER NOT NULL,
            sale_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            subtotal REAL NOT NULL DEFAULT 0,
            discount_amount REAL NOT NULL DEFAULT 0,
            vat_amount REAL NOT NULL DEFAULT 0,
            total_amount REAL NOT NULL DEFAULT 0,
            amount_paid REAL NOT NULL DEFAULT 0,
            change_amount REAL NOT NULL DEFAULT 0,
            payment_method TEXT NOT NULL DEFAULT 'cash'
                CHECK(payment_method IN ('cash','card','gcash','bank_transfer','credit')),
            status TEXT NOT NULL DEFAULT 'completed'
                CHECK(status IN ('completed','void','refunded')),
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (customer_id) REFERENCES customers(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        );

        -- Sale Items
        CREATE TABLE IF NOT EXISTS sale_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sale_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price REAL NOT NULL,
            discount_percent REAL NOT NULL DEFAULT 0,
            vat_amount REAL NOT NULL DEFAULT 0,
            total_price REAL NOT NULL,
            FOREIGN KEY (sale_id) REFERENCES sales(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        -- Invoices / Official Receipts
        CREATE TABLE IF NOT EXISTS invoices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sale_id INTEGER NOT NULL UNIQUE,
            invoice_number TEXT UNIQUE NOT NULL,
            invoice_date DATE NOT NULL,
            due_date DATE,
            status TEXT NOT NULL DEFAULT 'paid'
                CHECK(status IN ('paid','unpaid','partial','void')),
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (sale_id) REFERENCES sales(id)
        );

        -- Stock Adjustments
        CREATE TABLE IF NOT EXISTS stock_adjustments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            adjustment_type TEXT NOT NULL
                CHECK(adjustment_type IN ('add','subtract','set')),
            quantity_before INTEGER NOT NULL,
            quantity_change INTEGER NOT NULL,
            quantity_after INTEGER NOT NULL,
            reason TEXT,
            user_id INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        );

        -- Company Settings
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Default categories
        INSERT OR IGNORE INTO categories (name, description) VALUES
            ('LED Bulbs',    'LED light bulbs of all types and wattage'),
            ('Fluorescent',  'Fluorescent tubes and compact fluorescent lamps'),
            ('Downlights',   'Recessed and surface downlights'),
            ('Streetlights', 'Outdoor and street lighting fixtures'),
            ('Floodlights',  'High-power flood and area lighting'),
            ('Decorative',   'Decorative and novelty lighting'),
            ('Accessories',  'Electrical accessories and fittings');

        -- Default company settings
        INSERT OR IGNORE INTO settings (key, value) VALUES
            ('company_name',      'LumiSync Electronics'),
            ('company_address',   'Philippines'),
            ('company_phone',     ''),
            ('company_email',     ''),
            ('company_tin',       ''),
            ('currency_symbol',   '₱'),
            ('vat_rate',          '12'),
            ('invoice_prefix',    'OR'),
            ('sale_prefix',       'SL'),
            ('receipt_footer',    'Thank you for your business!'),
            ('low_stock_threshold','10');
        ",
    )?;

    // Insert admin user with dynamically generated hash
    let query = format!(
        "INSERT OR IGNORE INTO users (username, password_hash, full_name, role, email) \
         VALUES ('admin', '{}', 'System Administrator', 'super_admin', 'admin@lumisync.local')",
        password_hash.replace("'", "''") // Escape single quotes
    );
    conn.execute(&query, [])?;

    Ok(())
}

// Migration v2: Update admin password hash if it doesn't match
fn migration_v2(conn: &Connection) -> Result<()> {
    let password_hash = bcrypt::hash("Admin@123", 12)
        .unwrap_or_else(|_| "$2b$12$N9qo8uLOickgx2ZMRZoMyeIjZAgcg7b3XeKeUxWdeS86E36P4/aSi".to_string());
    
    let query = format!(
        "UPDATE users SET password_hash = '{}' WHERE username = 'admin'",
        password_hash.replace("'", "''")
    );
    conn.execute(&query, [])?;
    Ok(())
}

// Migration v3: Seed products across categories
fn migration_v3(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "INSERT OR IGNORE INTO products (category_id, sku, name, description, unit, cost_price, selling_price, quantity, reorder_level) VALUES
        (1, 'LED-12W-DL', 'LED Bulb 12W Daylight', 'Energy-efficient 12W LED bulb with daylight spectrum', 'piece', 25.00, 78.00, 45, 10),
        (1, 'LED-15W-WW', 'LED Bulb 15W Warm White', 'Warm white 15W LED bulb for ambient lighting', 'piece', 28.00, 89.00, 38, 10),
        (1, 'LED-9W-CW', 'LED Bulb 9W Cool White', 'Compact 9W cool white LED for task lighting', 'piece', 20.00, 65.00, 52, 10),
        (1, 'LED-20W-DIM', 'LED Bulb 20W Dimmable', 'Dimmable 20W LED for flexible lighting control', 'piece', 35.00, 120.00, 28, 10),
        (1, 'LED-5W-RGB', 'LED Bulb 5W RGB', 'Multicolor RGB LED bulb with 16 million colors', 'piece', 40.00, 145.00, 15, 10),
        (2, 'FL-T8-36W', 'Fluorescent Tube 36W T8', 'Standard 36W T8 fluorescent tube 120cm', 'piece', 15.00, 45.00, 60, 15),
        (2, 'FL-T5-28W', 'Fluorescent Tube 28W T5', 'Compact 28W T5 fluorescent tube 115cm', 'piece', 18.00, 52.00, 40, 15),
        (2, 'FL-CFL-23W', 'CFL Bulb 23W', 'Compact fluorescent lamp 23W equivalent 100W', 'piece', 12.00, 38.00, 75, 15),
        (2, 'FL-CFL-15W', 'CFL Bulb 15W', 'Compact fluorescent lamp 15W', 'piece', 8.00, 25.00, 85, 15),
        (2, 'FL-BALLAST', 'Electronic Ballast', 'Electronic ballast for fluorescent fixtures', 'piece', 45.00, 130.00, 20, 5),
        (3, 'DL-3INCH-LED', '3 Inch LED Downlight', 'Recessed 3\" LED downlight 8W daylight', 'piece', 50.00, 165.00, 25, 8),
        (3, 'DL-4INCH-LED', '4 Inch LED Downlight', 'Recessed 4\" LED downlight 12W warm white', 'piece', 65.00, 210.00, 18, 8),
        (3, 'DL-SURFACE-LED', 'Surface Mount Downlight', 'Surface mounted LED downlight 10W adjustable', 'piece', 55.00, 185.00, 22, 8),
        (3, 'DL-GIMBAL-3IN', 'Gimbal 3 Inch Downlight', 'Gimbal recessed downlight 3\" adjustable angle', 'piece', 70.00, 225.00, 15, 8),
        (3, 'DL-PANEL-300', 'LED Panel Light 30x30', 'Square LED panel light 30cm x 30cm 18W', 'piece', 75.00, 245.00, 12, 8),
        (4, 'SL-30W-LED', 'LED Street Light 30W', 'Energy-efficient 30W LED street light IP65', 'piece', 180.00, 580.00, 8, 3),
        (4, 'SL-50W-LED', 'LED Street Light 50W', 'High-power 50W LED street light IP67', 'piece', 280.00, 895.00, 5, 3),
        (4, 'SL-100W-HALIDE', 'Metal Halide 100W', 'Traditional 100W metal halide street light', 'piece', 120.00, 385.00, 12, 3),
        (4, 'SL-POLE-4M', 'Steel Pole 4 Meter', '4 meter galvanized steel lighting pole', 'piece', 450.00, 1350.00, 4, 2),
        (4, 'SL-BRACKET', 'Light Bracket', 'Adjustable mounting bracket for street lights', 'piece', 25.00, 75.00, 30, 10),
        (5, 'FL-100W-LED', 'LED Floodlight 100W', 'High-intensity 100W LED floodlight IP67', 'piece', 150.00, 495.00, 10, 5),
        (5, 'FL-150W-LED', 'LED Floodlight 150W', 'Professional 150W LED floodlight IP68', 'piece', 220.00, 720.00, 6, 5),
        (5, 'FL-300W-HALIDE', 'Metal Halide Floodlight 300W', '300W metal halide high-power floodlight', 'piece', 200.00, 650.00, 8, 3),
        (5, 'FL-STAND', 'Floodlight Stand', 'Adjustable tripod stand for floodlights', 'piece', 40.00, 130.00, 15, 5),
        (5, 'FL-DIFFUSER', 'Floodlight Diffuser', 'Optical diffuser for even light distribution', 'piece', 20.00, 65.00, 25, 10),
        (6, 'DEC-STRING-LIGHT', 'String Lights 10M', 'Decorative string lights 10 meters 20 bulbs', 'piece', 35.00, 115.00, 20, 5),
        (6, 'DEC-NEON-FLEX', 'Neon Flex Rope 5M', 'Flexible neon rope light 5 meters', 'piece', 80.00, 260.00, 10, 3),
        (6, 'DEC-SPOT-RGB', 'RGB Spot Light', 'Adjustable RGB accent light for decoration', 'piece', 30.00, 100.00, 35, 10),
        (6, 'DEC-CHANDELIER', 'Modern Chandelier', 'Contemporary chandelier with LED compatibility', 'piece', 200.00, 650.00, 5, 2),
        (6, 'DEC-PENDANT', 'Pendant Light', 'Stylish pendant light fixture', 'piece', 60.00, 195.00, 12, 5),
        (7, 'ACC-SWITCH-1G', 'Single Gang Switch', 'Standard single gang light switch', 'piece', 3.00, 12.00, 200, 30),
        (7, 'ACC-SWITCH-2G', 'Double Gang Switch', 'Double gang light switch plate', 'piece', 5.00, 18.00, 150, 30),
        (7, 'ACC-OUTLET-1G', 'Single Gang Outlet', 'Standard 15A electrical outlet', 'piece', 4.00, 15.00, 180, 30),
        (7, 'ACC-OUTLET-2G', 'Double Gang Outlet', 'Dual outlet 15A electrical outlet', 'piece', 6.00, 22.00, 120, 20),
        (7, 'ACC-SOCKET-E27', 'E27 Socket', 'Ceramic E27 bulb socket', 'piece', 2.00, 8.00, 300, 50),
        (7, 'ACC-SOCKET-B22', 'B22 Socket', 'Standard B22 bulb socket', 'piece', 2.50, 9.00, 250, 50),
        (7, 'ACC-WIRE-1MM', 'Electrical Wire 1.0mm 100M', '100 meters of 1.0mm electrical wire', 'roll', 45.00, 145.00, 35, 10),
        (7, 'ACC-WIRE-1.5MM', 'Electrical Wire 1.5mm 100M', '100 meters of 1.5mm electrical wire', 'roll', 65.00, 210.00, 28, 10),
        (7, 'ACC-JUNCTION-BOX', 'Junction Box', 'Electrical junction box 100x100mm', 'piece', 8.00, 28.00, 100, 20),
        (7, 'ACC-CONDUIT-PVC', 'PVC Conduit Pipe', 'PVC conduit pipe for wire protection 100M', 'roll', 35.00, 115.00, 25, 8),
        (7, 'ACC-BREAKER-16A', 'Circuit Breaker 16A', '16A automatic circuit breaker', 'piece', 12.00, 42.00, 80, 20),
        (7, 'ACC-SURGE-PROTECT', 'Surge Protector', '4-outlet surge protection power strip', 'piece', 15.00, 50.00, 90, 15);"
    )?;
    Ok(())
}

// Migration v4: Add pharmacy system tables
fn migration_v4(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        -- Pharmacy Patients
        CREATE TABLE IF NOT EXISTS pharmacy_patients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT,
            email TEXT,
            date_of_birth DATE,
            gender TEXT,
            address TEXT,
            allergies TEXT,
            medical_conditions TEXT,
            insurance_provider TEXT,
            insurance_member_id TEXT,
            preferred_pharmacy TEXT,
            emergency_contact TEXT,
            emergency_contact_phone TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Pharmacy Product Details (extends products table)
        CREATE TABLE IF NOT EXISTS pharmacy_product_details (
            product_id INTEGER PRIMARY KEY,
            generic_name TEXT,
            brand_name TEXT,
            strength TEXT,
            dosage_form TEXT,
            manufacturer TEXT,
            batch_number TEXT,
            expiry_date DATE,
            manufactured_date DATE,
            storage_condition TEXT,
            requires_prescription INTEGER NOT NULL DEFAULT 0,
            is_controlled_substance INTEGER NOT NULL DEFAULT 0,
            controlled_category TEXT,
            dea_number TEXT,
            is_covered_by_insurance INTEGER NOT NULL DEFAULT 0,
            insurance_tier INTEGER,
            ndc_code TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        -- Prescriptions
        CREATE TABLE IF NOT EXISTS prescriptions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            prescription_number TEXT UNIQUE NOT NULL,
            patient_id INTEGER NOT NULL,
            doctor_name TEXT,
            doctor_license TEXT,
            prescribing_date DATE NOT NULL,
            expiry_date DATE,
            instructions TEXT,
            refills_allowed INTEGER DEFAULT 0,
            refills_used INTEGER DEFAULT 0,
            is_controlled INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'filled', 'expired', 'voided')),
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (patient_id) REFERENCES pharmacy_patients(id)
        );

        -- Refill Requests
        CREATE TABLE IF NOT EXISTS refill_requests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            prescription_id INTEGER NOT NULL,
            patient_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            refill_number INTEGER NOT NULL,
            requested_date DATE NOT NULL,
            filled_date DATE,
            quantity INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'filled', 'denied', 'expired')),
            reason TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (prescription_id) REFERENCES prescriptions(id),
            FOREIGN KEY (patient_id) REFERENCES pharmacy_patients(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        -- Controlled Substance Log (for audit trail and compliance)
        CREATE TABLE IF NOT EXISTS controlled_substance_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            transaction_type TEXT NOT NULL CHECK(transaction_type IN ('sale', 'return', 'adjustment', 'destruction')),
            quantity INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            patient_id INTEGER,
            prescription_id INTEGER,
            witness_name TEXT,
            reason TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id),
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (patient_id) REFERENCES pharmacy_patients(id),
            FOREIGN KEY (prescription_id) REFERENCES prescriptions(id)
        );

        -- Pharmacy Sales (extends sales table with pharmacy-specific info)
        CREATE TABLE IF NOT EXISTS pharmacy_sales (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sale_id INTEGER NOT NULL UNIQUE,
            patient_id INTEGER,
            prescription_id INTEGER,
            pharmacist_name TEXT,
            pharmacist_license TEXT,
            insurance_covered REAL DEFAULT 0,
            insurance_claim_number TEXT,
            insurance_status TEXT DEFAULT 'pending' CHECK(insurance_status IN ('pending', 'approved', 'denied')),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (sale_id) REFERENCES sales(id) ON DELETE CASCADE,
            FOREIGN KEY (patient_id) REFERENCES pharmacy_patients(id),
            FOREIGN KEY (prescription_id) REFERENCES prescriptions(id)
        );

        -- Pharmacy Inventory Alerts
        CREATE TABLE IF NOT EXISTS pharmacy_inventory_alerts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            alert_type TEXT NOT NULL CHECK(alert_type IN ('low_stock', 'expiry_near', 'expired', 'out_of_stock')),
            alert_message TEXT NOT NULL,
            days_until_expiry INTEGER,
            quantity_threshold INTEGER,
            is_resolved INTEGER NOT NULL DEFAULT 0,
            resolved_by INTEGER,
            resolved_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id),
            FOREIGN KEY (resolved_by) REFERENCES users(id)
        );

        -- Update categories to include pharmacy categories
        INSERT OR IGNORE INTO categories (name, description) VALUES
            ('Antibiotics', 'Antibiotic medications'),
            ('Painkillers', 'Pain relief medications'),
            ('Cough & Cold', 'Cough and cold medicines'),
            ('Digestive', 'Digestive health products'),
            ('Vitamins & Supplements', 'Vitamins and dietary supplements'),
            ('Topical', 'Topical creams and ointments'),
            ('First Aid', 'First aid and wound care'),
            ('Over-the-Counter', 'OTC medications');
        "
    )?;
    Ok(())
}
