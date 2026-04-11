use rusqlite::{Connection, Result};
use std::path::Path;
use tauri::AppHandle;

pub fn get_db_path(app: &AppHandle) -> std::path::PathBuf {
    let app_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    app_dir.join("lumisync.db")
}

pub fn initialize_db(db_path: &Path) -> Result<Connection> {
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
        conn.execute(
            "INSERT INTO schema_migrations (version) VALUES (1)",
            [],
        )?;
    }

    Ok(())
}

fn migration_v1(conn: &Connection) -> Result<()> {
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

        -- Seed default super admin (password: Admin@123)
        INSERT OR IGNORE INTO users (username, password_hash, full_name, role)
        VALUES ('admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj9L8sDqhxBq', 'System Administrator', 'super_admin');

        -- Seed default company settings
        INSERT OR IGNORE INTO settings (key, value) VALUES
            ('company_name', 'LumiSync Electronics'),
            ('company_address', 'Philippines'),
            ('company_phone', ''),
            ('company_email', ''),
            ('company_tin', ''),
            ('currency_symbol', '₱'),
            ('vat_rate', '12'),
            ('invoice_prefix', 'OR'),
            ('sale_prefix', 'SL'),
            ('receipt_footer', 'Thank you for your business!'),
            ('low_stock_threshold', '10');

        -- Seed default categories
        INSERT OR IGNORE INTO categories (name, description) VALUES
            ('LED Bulbs', 'LED light bulbs of all types and wattage'),
            ('Fluorescent', 'Fluorescent tubes and compact fluorescent lamps'),
            ('Downlights', 'Recessed and surface downlights'),
            ('Streetlights', 'Outdoor and street lighting fixtures'),
            ('Floodlights', 'High-power flood and area lighting'),
            ('Decorative', 'Decorative and novelty lighting'),
            ('Accessories', 'Electrical accessories and fittings');
        ",
    )
}
