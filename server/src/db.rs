use rusqlite::{Connection, Result};
mod migration_v7;

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

    // Add migration v5 for pharmacy test data
    if current_version < 5 {
        migration_v5(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (5)", [])?;
    }

    // Add migration v6 for multi-industry support
    if current_version < 6 {
        migration_v6(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (6)", [])?;
    }

    // Add migration v7 for e-commerce
    if current_version < 7 {
        migration_v7::migration_v7(conn)?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (7)", [])?;
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

// Migration v6: Multi-industry support
fn migration_v6(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        -- Industries
        CREATE TABLE IF NOT EXISTS industries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT UNIQUE NOT NULL,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            icon TEXT,
            color TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Alter users table to add industry_id
        ALTER TABLE users ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;

        -- Alter products table to add industry_id
        ALTER TABLE products ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;

        -- Alter categories table to add industry_id (for industry-specific categories)
        ALTER TABLE categories ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;

        -- Alter settings to have industry_id (for multi-tenant company settings)
        ALTER TABLE settings ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;

        -- Product attributes table for industry-specific fields
        CREATE TABLE IF NOT EXISTS product_attributes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            industry_id INTEGER NOT NULL,
            attribute_name TEXT NOT NULL,
            attribute_label TEXT NOT NULL,
            attribute_type TEXT NOT NULL DEFAULT 'text'
                CHECK(attribute_type IN ('text', 'number', 'checkbox', 'select', 'date', 'textarea')),
            is_required INTEGER NOT NULL DEFAULT 0,
            display_order INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (industry_id) REFERENCES industries(id) ON DELETE CASCADE,
            UNIQUE(industry_id, attribute_name)
        );

        -- Product attribute values
        CREATE TABLE IF NOT EXISTS product_attribute_values (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            attribute_id INTEGER NOT NULL,
            attribute_value TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (attribute_id) REFERENCES product_attributes(id) ON DELETE CASCADE,
            UNIQUE(product_id, attribute_id)
        );
        "
    )?;

    // Insert default industries
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO industries (code, name, description, color) VALUES
            ('electronics', 'Electronics & Lighting', 'Retail of electronic devices and lighting fixtures', '#3B82F6'),
            ('pharmacy', 'Pharmacy & Healthcare', 'Pharmacy and healthcare products distribution', '#EC4899'),
            ('retail', 'General Retail', 'General retail and merchandise', '#8B5CF6'),
            ('grocery', 'Grocery & Food', 'Grocery stores and food retail', '#10B981'),
            ('clothing', 'Clothing & Fashion', 'Apparel and fashion retail', '#F59E0B'),
            ('furniture', 'Furniture & Home', 'Furniture and home goods retail', '#6366F1'),
            ('automotive', 'Automotive & Parts', 'Automotive sales and parts distribution', '#EF4444'),
            ('cosmetics', 'Cosmetics & Beauty', 'Beauty products and cosmetics retail', '#EC4899');
        "
    )?;

    // Migrate existing LumiSync data to electronics industry
    let electronics_id: i64 = conn
        .query_row(
            "SELECT id FROM industries WHERE code = 'electronics' LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(1);

    // Update existing users to electronics industry
    conn.execute(
        "UPDATE users SET industry_id = ? WHERE industry_id IS NULL",
        [electronics_id],
    )?;

    // Update existing products to electronics industry
    conn.execute(
        "UPDATE products SET industry_id = ? WHERE industry_id IS NULL",
        [electronics_id],
    )?;

    // Update existing categories to electronics industry (for non-pharmacy categories)
    conn.execute(
        "UPDATE categories SET industry_id = ? WHERE industry_id IS NULL AND name NOT IN (
            'Antibiotics', 'Painkillers', 'Cough & Cold', 'Digestive', 'Vitamins & Supplements',
            'Topical', 'First Aid', 'Over-the-Counter'
        )",
        [electronics_id],
    )?;

    // Migrate pharmacy categories to pharmacy industry
    let pharmacy_id: i64 = conn
        .query_row(
            "SELECT id FROM industries WHERE code = 'pharmacy' LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(2);

    conn.execute(
        "UPDATE categories SET industry_id = ? WHERE name IN (
            'Antibiotics', 'Painkillers', 'Cough & Cold', 'Digestive', 'Vitamins & Supplements',
            'Topical', 'First Aid', 'Over-the-Counter'
        )",
        [pharmacy_id],
    )?;

    // Define attributes for electronics industry
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO product_attributes (industry_id, attribute_name, attribute_label, attribute_type, is_required, display_order)
        SELECT id, 'wattage', 'Wattage (W)', 'text', 0, 1 FROM industries WHERE code = 'electronics' UNION ALL
        SELECT id, 'color_temp', 'Color Temperature', 'select', 0, 2 FROM industries WHERE code = 'electronics' UNION ALL
        SELECT id, 'lumens', 'Brightness (Lumens)', 'number', 0, 3 FROM industries WHERE code = 'electronics' UNION ALL
        SELECT id, 'lifespan_hours', 'Lifespan (hours)', 'number', 0, 4 FROM industries WHERE code = 'electronics';
        "
    )?;

    // Define attributes for pharmacy industry
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO product_attributes (industry_id, attribute_name, attribute_label, attribute_type, is_required, display_order)
        SELECT id, 'strength', 'Strength', 'text', 1, 1 FROM industries WHERE code = 'pharmacy' UNION ALL
        SELECT id, 'dosage_form', 'Dosage Form', 'select', 1, 2 FROM industries WHERE code = 'pharmacy' UNION ALL
        SELECT id, 'manufacturer', 'Manufacturer', 'text', 1, 3 FROM industries WHERE code = 'pharmacy' UNION ALL
        SELECT id, 'expiry_date', 'Expiry Date', 'date', 1, 4 FROM industries WHERE code = 'pharmacy';
        "
    )?;

    Ok(())
}

// Migration v5: Seed pharmacy test data
fn migration_v5(conn: &Connection) -> Result<()> {
    // Insert test pharmacy products
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO products (category_id, sku, name, description, unit, cost_price, selling_price, quantity, reorder_level)
        VALUES
        -- Antibiotics (Category 8)
        (8, 'AMOX-500', 'Amoxicillin 500mg', 'Broad-spectrum antibiotic', 'tablet', 2.50, 12.00, 100, 20),
        (8, 'CIPRO-500', 'Ciprofloxacin 500mg', 'Fluoroquinolone antibiotic', 'tablet', 3.50, 18.00, 80, 20),
        (8, 'AZI-250', 'Azithromycin 250mg', 'Macrolide antibiotic', 'tablet', 4.00, 22.00, 60, 15),
        
        -- Painkillers (Category 9)
        (9, 'PARACET-500', 'Paracetamol 500mg', 'Pain reliever and fever reducer', 'tablet', 1.50, 8.00, 200, 50),
        (9, 'IBUPROF-200', 'Ibuprofen 200mg', 'Anti-inflammatory pain reliever', 'tablet', 2.00, 10.00, 150, 40),
        (9, 'ASPIRIN-500', 'Aspirin 500mg', 'Analgesic and anti-inflammatory', 'tablet', 1.80, 9.00, 120, 30),
        
        -- Cough & Cold (Category 10)
        (10, 'COUGH-SYR', 'Cough Syrup 100ml', 'Cough suppressant', 'bottle', 5.00, 25.00, 50, 10),
        (10, 'LOZEN-20', 'Throat Lozenges 20pc', 'Sore throat relief', 'pack', 2.50, 12.00, 80, 20),
        (10, 'DECON-TAB', 'Decongestant Tablet', 'Nasal decongestant', 'tablet', 1.20, 6.00, 200, 50),
        
        -- Digestive (Category 11)
        (11, 'ANTACID-30', 'Antacid 30ml', 'Heartburn relief', 'bottle', 3.00, 15.00, 100, 20),
        (11, 'LACTAID-100', 'Lactase Enzyme 100pc', 'Lactose intolerance aid', 'capsule', 8.00, 35.00, 40, 10),
        (11, 'PROBIO-50', 'Probiotic 50 capsules', 'Gut health supplement', 'bottle', 12.00, 60.00, 30, 10),
        
        -- Vitamins & Supplements (Category 12)
        (12, 'VITC-500', 'Vitamin C 500mg', 'Immunity booster', 'tablet', 3.00, 15.00, 150, 30),
        (12, 'MULTIVIT', 'Multivitamin Daily', 'Complete vitamin complex', 'tablet', 4.00, 20.00, 100, 20),
        (12, 'CALCIUM-600', 'Calcium 600mg', 'Bone health supplement', 'tablet', 2.50, 12.00, 120, 25),
        (12, 'IRON-PLUS', 'Iron with Folic Acid', 'Anemia treatment', 'tablet', 3.50, 18.00, 80, 20),
        
        -- Topical (Category 13)
        (13, 'NEOSPORIN-30', 'Neosporin 30g', 'Antibiotic ointment', 'tube', 4.00, 18.00, 100, 20),
        (13, 'HYDROCORT-15', 'Hydrocortisone 1% 15g', 'Anti-inflammatory cream', 'tube', 5.00, 22.00, 70, 15),
        (13, 'LOTION-200', 'Moisturizing Lotion 200ml', 'Dry skin treatment', 'bottle', 6.00, 28.00, 60, 15),
        
        -- First Aid (Category 14)
        (14, 'BANDAGE-BOX', 'Adhesive Bandages Box', '100 count sterile bandages', 'box', 4.00, 18.00, 90, 20),
        (14, 'GAUZE-PAD', 'Sterile Gauze Pads', 'Wound care sterile pads', 'pack', 3.50, 16.00, 120, 25),
        (14, 'TAPE-MED', 'Medical Tape Roll', 'Hypoallergenic tape', 'roll', 2.50, 12.00, 150, 30);
        "
    )?;

    // Insert test pharmacy product details with expiry dates
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO pharmacy_product_details (product_id, generic_name, brand_name, strength, dosage_form, manufacturer, batch_number, expiry_date, requires_prescription, is_controlled_substance, is_covered_by_insurance)
        SELECT id, 'Amoxicillin', 'Amoxil', '500mg', 'tablet', 'Pharma Corp', 'BATCH001', '2026-08-15', 1, 0, 1 FROM products WHERE sku = 'AMOX-500' UNION ALL
        SELECT id, 'Ciprofloxacin', 'Cipro', '500mg', 'tablet', 'Pharma Corp', 'BATCH002', '2026-09-20', 1, 0, 1 FROM products WHERE sku = 'CIPRO-500' UNION ALL
        SELECT id, 'Azithromycin', 'Zithromax', '250mg', 'tablet', 'Pharma Corp', 'BATCH003', '2026-07-10', 1, 0, 1 FROM products WHERE sku = 'AZI-250' UNION ALL
        SELECT id, 'Paracetamol', 'Tylenol', '500mg', 'tablet', 'Pharma Corp', 'BATCH004', '2026-12-30', 0, 0, 1 FROM products WHERE sku = 'PARACET-500' UNION ALL
        SELECT id, 'Ibuprofen', 'Advil', '200mg', 'tablet', 'Pharma Corp', 'BATCH005', '2026-11-15', 0, 0, 1 FROM products WHERE sku = 'IBUPROF-200' UNION ALL
        SELECT id, 'Aspirin', 'Bayer', '500mg', 'tablet', 'Pharma Corp', 'BATCH006', '2026-10-25', 0, 0, 1 FROM products WHERE sku = 'ASPIRIN-500' UNION ALL
        SELECT id, 'Dextromethorphan', 'Robitussin', '100ml', 'liquid', 'Pharma Corp', 'BATCH007', '2026-06-30', 0, 1, 1 FROM products WHERE sku = 'COUGH-SYR' UNION ALL
        SELECT id, 'Menthol', 'Halls', '20pc', 'lozenge', 'Pharma Corp', 'BATCH008', '2026-05-15', 0, 0, 0 FROM products WHERE sku = 'LOZEN-20' UNION ALL
        SELECT id, 'Phenylephrine', 'Sudafed', 'tablet', 'tablet', 'Pharma Corp', 'BATCH009', '2026-04-20', 1, 0, 1 FROM products WHERE sku = 'DECON-TAB' UNION ALL
        SELECT id, 'Calcium Carbonate', 'Tums', '30ml', 'liquid', 'Pharma Corp', 'BATCH010', '2026-08-10', 0, 0, 0 FROM products WHERE sku = 'ANTACID-30';
        "
    )?;

    // Insert test pharmacy patients
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO pharmacy_patients (name, phone, email, date_of_birth, gender, address, allergies, medical_conditions, insurance_provider, insurance_member_id, emergency_contact, emergency_contact_phone)
        VALUES
        ('Maria Santos', '09123456789', 'maria@email.com', '1975-05-15', 'F', '123 Main St, Manila', 'Penicillin, Sulfa', 'Hypertension, Diabetes', 'PhilHealth', 'PH-001-2023', 'Juan Santos', '09198765432'),
        ('Juan Dela Cruz', '09234567890', 'juan@email.com', '1980-08-22', 'M', '456 Oak Ave, Cebu', 'Aspirin', 'Asthma', 'PhilHealth', 'PH-002-2023', 'Rosa Dela Cruz', '09187654321'),
        ('Rosa Garcia', '09345678901', 'rosa@email.com', '1988-03-10', 'F', '789 Pine Rd, Davao', 'Shellfish', 'Arthritis', 'PhilCare', 'PC-001-2023', 'Carlos Garcia', '09176543210'),
        ('Carlos Reyes', '09456789012', 'carlos@email.com', '1965-11-30', 'M', '321 Elm St, Quezon City', 'None', 'Hypertension, High Cholesterol', 'PhilHealth', 'PH-003-2023', 'Maria Reyes', '09165432109'),
        ('Anna Martinez', '09567890123', 'anna@email.com', '1992-07-18', 'F', '654 Birch Ln, Makati', 'Peanuts', 'None', 'Amcare', 'AM-001-2023', 'Miguel Martinez', '09154321098');
        "
    )?;

    // Insert test prescriptions
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO prescriptions (prescription_number, patient_id, doctor_name, doctor_license, prescribing_date, expiry_date, instructions, refills_allowed, is_controlled, status, notes)
        VALUES
        ('RX001-2026', 1, 'Dr. Jose Santos', 'LIC-001', '2026-04-10', '2026-10-10', 'Take 1 tablet twice daily for 7 days', 2, 1, 'active', 'For infection'),
        ('RX002-2026', 2, 'Dr. Maria Gonzales', 'LIC-002', '2026-04-08', '2026-10-08', 'Take 2 tablets twice daily as needed for pain', 0, 0, 'active', 'For chronic pain'),
        ('RX003-2026', 3, 'Dr. Robert Torres', 'LIC-003', '2026-04-12', '2026-04-30', 'Take 1 tablet daily for high blood pressure', 5, 0, 'active', 'Maintenance'),
        ('RX004-2026', 4, 'Dr. Angela Cruz', 'LIC-004', '2026-03-15', '2026-09-15', 'Take 1 capsule twice daily for cholesterol', 3, 0, 'active', 'Long-term therapy'),
        ('RX005-2026', 5, 'Dr. Jose Santos', 'LIC-001', '2026-04-13', '2026-05-13', 'Take 1 tablet every 4 hours as needed', 1, 1, 'active', 'For acute pain');
        "
    )?;

    Ok(())
}
