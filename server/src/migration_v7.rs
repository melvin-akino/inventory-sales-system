// Migration v7: E-commerce tables (customers, orders, order_items, payments)
pub fn migration_v7(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        -- E-commerce Customers (separate from POS customers)
        CREATE TABLE IF NOT EXISTS ecommerce_customers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            phone TEXT,
            shipping_address TEXT,
            shipping_city TEXT,
            shipping_state TEXT,
            shipping_zip TEXT,
            shipping_country TEXT,
            billing_address TEXT,
            billing_city TEXT,
            billing_state TEXT,
            billing_zip TEXT,
            billing_country TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- E-commerce Sessions (separate from admin sessions)
        CREATE TABLE IF NOT EXISTS ecommerce_sessions (
            token TEXT PRIMARY KEY,
            customer_id INTEGER NOT NULL,
            expires_at DATETIME NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (customer_id) REFERENCES ecommerce_customers(id) ON DELETE CASCADE
        );

        -- Orders (e-commerce only)
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_number TEXT UNIQUE NOT NULL,
            customer_id INTEGER NOT NULL,
            order_status TEXT NOT NULL DEFAULT 'pending'
                CHECK(order_status IN ('pending', 'processing', 'shipped', 'delivered', 'cancelled', 'refunded')),
            payment_status TEXT NOT NULL DEFAULT 'unpaid'
                CHECK(payment_status IN ('unpaid', 'paid', 'failed', 'refunded')),
            payment_method TEXT NOT NULL DEFAULT 'card'
                CHECK(payment_method IN ('card', 'gcash', 'maya', 'bank_transfer')),
            subtotal REAL NOT NULL DEFAULT 0,
            shipping_cost REAL NOT NULL DEFAULT 0,
            tax_amount REAL NOT NULL DEFAULT 0,
            discount_amount REAL NOT NULL DEFAULT 0,
            total_amount REAL NOT NULL DEFAULT 0,
            shipping_address TEXT,
            shipping_city TEXT,
            shipping_state TEXT,
            shipping_zip TEXT,
            shipping_country TEXT,
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (customer_id) REFERENCES ecommerce_customers(id) ON DELETE CASCADE
        );

        -- Order Items
        CREATE TABLE IF NOT EXISTS order_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price REAL NOT NULL,
            total_price REAL NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        -- Payments (mock and real)
        CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            payment_method TEXT NOT NULL DEFAULT 'card'
                CHECK(payment_method IN ('card', 'gcash', 'maya', 'bank_transfer', 'mock')),
            payment_status TEXT NOT NULL DEFAULT 'pending'
                CHECK(payment_status IN ('pending', 'completed', 'failed', 'refunded')),
            payment_gateway TEXT,
            transaction_id TEXT,
            reference_number TEXT,
            error_message TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
        );

        -- Shopping Carts (session-based)
        CREATE TABLE IF NOT EXISTS shopping_carts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_id INTEGER,
            session_token TEXT,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (customer_id) REFERENCES ecommerce_customers(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        -- Create index for better query performance
        CREATE INDEX IF NOT EXISTS idx_orders_customer_id ON orders(customer_id);
        CREATE INDEX IF NOT EXISTS idx_orders_order_number ON orders(order_number);
        CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
        CREATE INDEX IF NOT EXISTS idx_payments_order_id ON payments(order_id);
        CREATE INDEX IF NOT EXISTS idx_shopping_carts_customer_id ON shopping_carts(customer_id);
        "
    )?;
    Ok(())
}
