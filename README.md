# LumiSync — Inventory & Sales System
## For Philippine Electronics & Lighting Businesses

A full-featured inventory, sales, and invoicing system built with **Tauri/Rust**, **SQLite**, and **Vue.js**. Designed for electronics shops selling bulbs and lighting equipment in the Philippines.

---

## ✨ Features

### Inventory Management
- Product catalog with SKU, categories, pricing, stock levels
- Stock adjustment (add/subtract/set) with audit trail
- Low stock alerts with configurable reorder levels
- Multi-category support (LED Bulbs, Fluorescent, Downlights, Streetlights, etc.)

### Point of Sale
- Fast product search & cart management
- Per-item discount percentage
- Automatic VAT (12%) calculation (VAT-exempt items supported)
- Multiple payment methods: Cash, Card, GCash, Bank Transfer, Store Credit
- Change computation
- Walk-in and named customer support

### Sales & Invoicing
- Auto-generated Official Receipts (OR numbers) per BIR format
- Sales history with full detail view
- Void/reverse sales with automatic stock restoration
- Printable receipts and invoices

### Accounting Reports
| Report | Access |
|--------|--------|
| Sales Report (by date range) | Manager+ |
| Inventory Valuation Report | Manager+ |
| Profit & Loss by Category | Admin+ |
| VAT Output Report (BIR) | Admin+ |

### User Management & Access Control
| Role | Capabilities |
|------|-------------|
| **Super Admin** | Full system access, user management, settings |
| **Admin** | Inventory, sales, reports, customers, suppliers |
| **Manager** | Inventory, sales, reports, customers |
| **Cashier** | Point of Sale, view invoices, view own sales |
| **Viewer** | Read-only access to inventory and sales |

---

## 🚀 Quick Start (Single Command)

```bash
# Clone & setup
git clone <repository-url>
cd inventory-sales-system
chmod +x setup.sh

# Desktop app (default) — recommended
./setup.sh

# or explicitly:
./setup.sh desktop        # Build & run desktop app (Tauri)
./setup.sh dev            # Development mode with hot-reload
./setup.sh web            # Build & serve as web app
./setup.sh dev-web        # Web development mode
```

### First Login
```
Username: admin
Password: Admin@123
```
> Change the default password immediately after first login via **User Management → Change My Password**

---

## 🛠 Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Framework | [Tauri](https://tauri.app) v1.6 |
| Backend Language | Rust (stable) |
| Database | SQLite (via `rusqlite`, bundled) |
| Frontend | Vue.js 3 + Vite |
| State Management | Pinia |
| Styling | Tailwind CSS |
| Icons | Unicode Emoji |

---

## 📋 System Requirements

### Desktop (Tauri)
- **Linux**: Ubuntu 18.04+, Debian 10+, or compatible
- **macOS**: 10.15+ (Catalina)
- **Windows**: Windows 10+

The setup script automatically installs:
- Node.js (via nvm if not present)
- Rust (via rustup if not present)
- Required Linux system libraries

### Web Mode
- Node.js 18+
- Any modern browser

---

## 📁 Project Structure

```
inventory-sales-system/
├── setup.sh                    # One-command setup
├── package.json                # Node.js dependencies
├── vite.config.js              # Vite build configuration
├── tailwind.config.js          # Tailwind CSS config
├── index.html                  # App entry point
│
├── src/                        # Vue.js Frontend
│   ├── main.js
│   ├── App.vue
│   ├── router/index.js         # Vue Router
│   ├── stores/
│   │   ├── auth.js             # Authentication state
│   │   └── app.js             # App-wide state & notifications
│   ├── utils/
│   │   ├── api.js             # Tauri IPC / REST API bridge
│   │   └── format.js          # Formatters, constants, roles
│   ├── views/
│   │   ├── Login.vue
│   │   ├── Dashboard.vue
│   │   ├── inventory/          # Products, Categories, Stock
│   │   ├── sales/              # POS, Sales History
│   │   ├── invoices/           # Official Receipts
│   │   ├── customers/
│   │   ├── suppliers/
│   │   ├── reports/            # Sales, Inventory, P&L, VAT
│   │   ├── users/              # User Management
│   │   └── Settings.vue
│   └── components/
│       ├── layout/             # Sidebar, Header, NavItem
│       └── common/             # Modal, ConfirmDialog
│
└── src-tauri/                  # Rust/Tauri Backend
    ├── Cargo.toml
    ├── tauri.conf.json
    └── src/
        ├── main.rs             # Tauri entry point
        ├── db/mod.rs           # DB init & migrations
        ├── models/             # Data structures
        └── commands/           # Business logic
            ├── auth.rs         # Login, sessions, RBAC
            ├── inventory.rs    # Products, categories, stock
            ├── sales.rs        # POS, sales, void
            ├── invoices.rs     # OR/Invoice queries
            ├── customers.rs
            ├── suppliers.rs
            ├── reports.rs      # All accounting reports
            ├── users.rs        # User CRUD
            └── settings.rs     # Company settings
```

---

## 🇵🇭 Philippines-Specific Features

- **Currency**: Philippine Peso (₱)
- **VAT**: 12% (configurable, BIR standard)
- **VAT Report**: Formatted for BIR output tax filing
- **TIN Number**: Supported on customer and company records
- **Official Receipts**: Auto-generated OR numbers with configurable prefix
- **Payment Methods**: Cash, Card, GCash, Bank Transfer, Store Credit

---

## 🔒 Security

- Passwords hashed with bcrypt (cost factor 12)
- Session tokens (UUID v4) stored in SQLite with expiry
- Role-based access control on every API command
- Sessions expire after 8 hours

---

## 🗄 Database

Data is stored in a SQLite file at:
- **Linux/macOS**: `~/.local/share/lumisync/lumisync.db` (or OS app data dir)
- **Windows**: `%APPDATA%\lumisync\lumisync.db`

The database is auto-created and migrated on first launch.

---

## 📄 License

Proprietary — All rights reserved. For use by licensed clients only.
