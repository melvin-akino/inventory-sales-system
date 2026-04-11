# LumiSync — Administrator Guide
**Version 1.0**

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Installation & Setup](#2-installation--setup)
3. [Configuration](#3-configuration)
4. [Database Management](#4-database-management)
5. [User Administration](#5-user-administration)
6. [Security](#6-security)
7. [Backup & Recovery](#7-backup--recovery)
8. [Deployment Modes](#8-deployment-modes)
9. [Troubleshooting](#9-troubleshooting)

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────┐
│                  LumiSync                        │
├───────────────────────┬─────────────────────────┤
│   Frontend (Vue.js)   │   Backend (Rust/Tauri)  │
│                       │                         │
│  • Vue 3 + Vite       │  • Tauri v1.6           │
│  • Pinia state mgmt   │  • Rust (stable)        │
│  • Vue Router         │  • rusqlite (SQLite)    │
│  • Tailwind CSS       │  • bcrypt auth          │
│                       │  • UUID sessions        │
└───────────────────────┴─────────────────────────┘
                        │
              ┌─────────▼─────────┐
              │   SQLite Database  │
              │   lumisync.db      │
              └───────────────────┘
```

### Technology Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Desktop wrapper | Tauri | 1.6 |
| Backend language | Rust | stable |
| Database | SQLite (bundled) | 3.x |
| Frontend framework | Vue.js | 3.3 |
| Build tool | Vite | 5.x |
| CSS framework | Tailwind CSS | 3.x |
| State management | Pinia | 2.x |

---

## 2. Installation & Setup

### Prerequisites

| Tool | Minimum Version | Install |
|------|----------------|---------|
| Node.js | 18.x LTS | https://nodejs.org |
| Rust | 1.70+ | https://rustup.rs |
| npm/pnpm | Latest | Bundled with Node.js |

**Linux additional packages:**
```bash
sudo apt-get install -y \
  libwebkit2gtk-4.0-dev build-essential curl wget \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**macOS:** Install Xcode Command Line Tools:
```bash
xcode-select --install
```

---

### One-Command Setup

```bash
git clone <repository-url>
cd inventory-sales-system
chmod +x setup.sh

# Desktop app (recommended)
./setup.sh

# Development mode
./setup.sh dev

# Web mode (static build)
./setup.sh web
```

The setup script automatically:
1. Detects and installs Node.js (via nvm) if missing
2. Detects and installs Rust (via rustup) if missing
3. Installs Linux system libraries if on Ubuntu/Debian
4. Runs `npm install`
5. Builds the application

---

## 3. Configuration

### Environment

The application requires no `.env` file for standard operation. All runtime configuration is stored in the SQLite `settings` table.

### Settings Table

Managed via **System Settings** (Super Admin only) or directly via SQLite:

| Key | Default | Description |
|-----|---------|-------------|
| `company_name` | LumiSync Electronics | Company name on receipts |
| `company_address` | Philippines | Company address |
| `company_phone` | (empty) | Contact number |
| `company_email` | (empty) | Contact email |
| `company_tin` | (empty) | BIR TIN number |
| `currency_symbol` | ₱ | Currency symbol |
| `vat_rate` | 12 | VAT percentage |
| `invoice_prefix` | OR | OR number prefix |
| `sale_prefix` | SL | Sale number prefix |
| `receipt_footer` | Thank you… | Receipt bottom text |
| `low_stock_threshold` | 10 | Default reorder alert |

---

## 4. Database Management

### Database Location

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/lumisync/lumisync.db` |
| macOS | `~/Library/Application Support/com.lumisync.inventory/lumisync.db` |
| Windows | `%APPDATA%\com.lumisync.inventory\lumisync.db` |

### Schema

The database uses WAL (Write-Ahead Logging) mode and enforces foreign key constraints.

**Tables:**

| Table | Purpose |
|-------|---------|
| `users` | User accounts and credentials |
| `sessions` | Active login sessions |
| `categories` | Product categories |
| `products` | Product catalogue |
| `suppliers` | Supplier records |
| `customers` | Customer records |
| `sales` | Sales transactions header |
| `sale_items` | Line items for each sale |
| `invoices` | Official Receipt records |
| `stock_adjustments` | Stock change audit log |
| `settings` | Key-value configuration |
| `schema_migrations` | DB migration version tracking |

### Manual Database Access

```bash
# Install SQLite3 CLI
sudo apt install sqlite3

# Open the database
sqlite3 ~/.local/share/lumisync/lumisync.db

# Useful queries
SELECT * FROM users;
SELECT * FROM sales ORDER BY created_at DESC LIMIT 10;
SELECT SUM(total_amount) FROM sales WHERE status='completed' AND DATE(sale_date)=DATE('now');
```

### Schema Migrations

Migrations run automatically on startup. Each migration is applied only once and tracked in the `schema_migrations` table. New migrations are added in `src-tauri/src/db/mod.rs`.

---

## 5. User Administration

### Default Admin Account

```
Username: admin
Password: Admin@123
Role:     super_admin
```

**Change the password immediately after deployment.**

### Password Security

- Passwords are hashed with **bcrypt** (cost factor 12)
- Minimum length: 6 characters
- No maximum length enforced by the system

### Session Management

- Sessions use **UUID v4** tokens
- Session lifetime: **8 hours**
- Old sessions are cleared on new login (one session per user)
- Sessions stored in the `sessions` table

### Force Logout a User

```sql
DELETE FROM sessions WHERE user_id = (SELECT id FROM users WHERE username = 'targetuser');
```

### Reset a Forgotten Password (Direct DB)

```bash
# Generate a new bcrypt hash (Python example)
python3 -c "import bcrypt; print(bcrypt.hashpw(b'NewPassword123', bcrypt.gensalt(12)).decode())"

# Apply to database
sqlite3 lumisync.db "UPDATE users SET password_hash='<hash>' WHERE username='admin';"
```

---

## 6. Security

### Authentication

- bcrypt password hashing (cost 12)
- UUID v4 session tokens (128-bit entropy)
- Session expiry: 8 hours
- No password in session storage — only the token

### Authorization

Every Tauri command validates:
1. The session token (valid + not expired + user active)
2. The user's role against the required role list

Unauthorized calls return an error — they do not silently fail.

### Data Storage

- SQLite database is stored in the OS app data directory
- No cloud sync, no telemetry, no external network calls
- All data remains on the local machine (desktop mode)

### Input Sanitization

- SQL queries use parameterized statements throughout (`rusqlite` `params![]`)
- No raw SQL string interpolation with user input in the core logic
- String-interpolated SQL (in filter queries) sanitizes single-quote characters

---

## 7. Backup & Recovery

### Manual Backup

Copy the database file while the application is closed (or in WAL mode, copying while running is also safe):

```bash
# Linux/macOS
cp ~/.local/share/lumisync/lumisync.db ~/backups/lumisync-$(date +%Y%m%d).db

# Windows (PowerShell)
Copy-Item "$env:APPDATA\com.lumisync.inventory\lumisync.db" "C:\Backups\lumisync-$(Get-Date -Format yyyyMMdd).db"
```

### Automated Backup (Linux cron)

```bash
# Add to crontab (crontab -e) — daily backup at midnight
0 0 * * * cp ~/.local/share/lumisync/lumisync.db ~/backups/lumisync-$(date +\%Y\%m\%d).db
```

### Restore from Backup

1. Close LumiSync
2. Replace the database file with the backup:
   ```bash
   cp ~/backups/lumisync-20240415.db ~/.local/share/lumisync/lumisync.db
   ```
3. Reopen LumiSync

### Export Data to CSV (via SQLite)

```bash
sqlite3 -header -csv lumisync.db "SELECT * FROM sales;" > sales_export.csv
sqlite3 -header -csv lumisync.db "SELECT * FROM products;" > products_export.csv
```

---

## 8. Deployment Modes

### Mode 1: Desktop Application (Recommended)

Build a native installer:

```bash
./setup.sh desktop
# or
npm run tauri build
```

Output locations:
- **Linux (AppImage):** `src-tauri/target/release/bundle/appimage/`
- **Linux (deb):** `src-tauri/target/release/bundle/deb/`
- **macOS (dmg):** `src-tauri/target/release/bundle/dmg/`
- **Windows (msi/nsis):** `src-tauri/target/release/bundle/msi/` or `nsis/`

**Advantages:**
- No server required
- Database stored locally
- Faster performance
- Works offline

---

### Mode 2: Web Application

Build the static frontend:

```bash
./setup.sh web
# or
npm run build && serve dist -l 8080
```

**For a production web deployment with a real backend**, you would need to:
1. Build the frontend (`npm run build`)
2. Set up a web server (nginx, Apache) to serve the `dist/` folder
3. Implement a REST API backend that replicates the Tauri commands

> **Note:** The current web mode serves the built frontend statically. The Tauri IPC backend is not available in web mode — a separate REST API server would be needed for full functionality.

---

### Mode 3: Development

```bash
./setup.sh dev        # Tauri dev (hot reload, desktop)
./setup.sh dev-web    # Vite dev server only (frontend, no backend)
```

---

## 9. Troubleshooting

### Build Errors

**`error: linker 'cc' not found`**
```bash
sudo apt install build-essential
```

**`webkit2gtk not found`**
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

**`error[E0433]: failed to resolve: use of undeclared crate`**
- Run `cargo update` in `src-tauri/`
- Check `Cargo.toml` dependencies

---

### Runtime Errors

**`Failed to initialize database`**
- Check write permissions to the app data directory
- Ensure disk space is available

**`UNIQUE constraint failed`**
- A duplicate username or SKU was entered
- Choose a unique value

**`Session expired`**
- Log in again — sessions expire after 8 hours

---

### Performance

- For databases over 100,000 sales records, consider archiving old data
- WAL mode (enabled by default) provides good concurrent read performance
- Add indexes if custom queries are slow:
  ```sql
  CREATE INDEX IF NOT EXISTS idx_sales_date ON sales(sale_date);
  CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
  ```

---

*LumiSync v1.0 Admin Guide*
