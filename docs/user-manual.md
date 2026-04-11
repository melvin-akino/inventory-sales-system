# LumiSync — User Manual
**Version 1.0 | Philippine Electronics & Lighting Inventory System**

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Logging In](#2-logging-in)
3. [Dashboard](#3-dashboard)
4. [Inventory Management](#4-inventory-management)
   - 4.1 [Categories](#41-categories)
   - 4.2 [Products](#42-products)
   - 4.3 [Stock Adjustments](#43-stock-adjustments)
5. [Point of Sale (POS)](#5-point-of-sale-pos)
6. [Sales History](#6-sales-history)
7. [Invoices / Official Receipts](#7-invoices--official-receipts)
8. [Customers](#8-customers)
9. [Suppliers](#9-suppliers)
10. [Reports](#10-reports)
    - 10.1 [Sales Report](#101-sales-report)
    - 10.2 [Inventory Report](#102-inventory-report)
    - 10.3 [Profit & Loss](#103-profit--loss-report)
    - 10.4 [VAT Report (BIR)](#104-vat-report-bir)
11. [User Management](#11-user-management)
12. [System Settings](#12-system-settings)
13. [Access Control Reference](#13-access-control-reference)

---

## 1. Introduction

**LumiSync** is an all-in-one inventory, sales, and invoicing system designed for Philippine electronics and lighting businesses. It tracks stock levels, processes sales transactions, generates Official Receipts (OR) compliant with BIR requirements, and provides comprehensive accounting reports.

### Key Features
- Product catalogue with categories and SKU management
- Real-time stock tracking with low-stock alerts
- Point-of-Sale (POS) with VAT computation and multiple payment methods
- Auto-generated Official Receipts (OR) with sequential numbering
- Customer and supplier records with TIN support
- Sales, Inventory, Profit & Loss, and VAT reports
- Multi-user system with role-based access control
- Available as desktop application (Tauri) or web application

---

## 2. Logging In

1. Launch the LumiSync application.
2. Enter your **Username** and **Password**.
3. Click **Sign In**.

**Default credentials (change immediately after first login):**
```
Username: admin
Password: Admin@123
```

Sessions expire automatically after 8 hours. You will be redirected to the login screen when your session ends.

---

## 3. Dashboard

The Dashboard provides an at-a-glance view of business performance:

| Widget | Description |
|--------|-------------|
| Sales Today | Total revenue and number of transactions for the current day |
| This Month | Cumulative sales revenue for the current calendar month |
| Products | Total number of active products in the catalogue |
| Low Stock | Count of products at or below their reorder level (click to view) |
| Customers | Total registered customers |
| Top Products | Top 5 best-selling products this month by quantity |
| Recent Sales | Last 10 completed transactions |

---

## 4. Inventory Management

### 4.1 Categories

**Path:** Inventory → Categories  
**Access:** Manager, Admin, Super Admin

Categories organise your product catalogue. Pre-loaded categories include:
- LED Bulbs, Fluorescent, Downlights, Streetlights, Floodlights, Decorative, Accessories

**Adding a category:**
1. Click **+ Add Category**
2. Enter the name and optional description
3. Click **Save**

---

### 4.2 Products

**Path:** Products  
**Access:** View — All roles | Add/Edit/Delete — Manager+

#### Product Fields

| Field | Required | Description |
|-------|----------|-------------|
| Product Name | ✓ | Full product description |
| SKU | ✓ | Unique stock-keeping unit code |
| Category | | Organises products into groups |
| Unit of Measure | ✓ | piece, box, set, meter, roll |
| Cost Price | ✓ | Purchase cost (used in P&L reports) |
| Selling Price | ✓ | Price charged to customers |
| Initial Stock | | Starting quantity when creating a product |
| Reorder Level | ✓ | Alert threshold (default: 10) |
| VAT Exempt | | Check for products not subject to 12% VAT |

#### Stock Status Badges

| Badge | Condition |
|-------|-----------|
| 🟢 In Stock | Quantity > Reorder Level |
| 🟡 Low Stock | Quantity ≤ Reorder Level (but > 0) |
| 🔴 Out of Stock | Quantity = 0 |

#### Filtering Products
- **Search:** Name or SKU
- **Category:** Dropdown filter
- **Low Stock Only:** Shows only products at or below reorder level
- **Show Inactive:** Includes deactivated products

---

### 4.3 Stock Adjustments

**Path:** Inventory → Stock Adjustments  
**Access:** Manager, Admin, Super Admin

Use stock adjustments to update quantities outside of normal sales:

| Adjustment Type | When to Use |
|----------------|-------------|
| **Add** | Received stock from supplier |
| **Subtract** | Damaged, expired, or written-off stock |
| **Set** | Physical stocktake — set exact count |

All adjustments are recorded in an audit log showing the before/after quantities, reason, date, and the user who made the adjustment.

---

## 5. Point of Sale (POS)

**Path:** Sales → Point of Sale  
**Access:** Cashier, Manager, Admin, Super Admin

### Processing a Sale

1. **Search products** — type in the search bar or filter by category
2. **Add to cart** — click any product card (disabled if out of stock)
3. **Adjust cart items:**
   - Change quantity with **+** / **−** buttons
   - Enter item discount percentage
   - Remove items with **✕**
4. **Select customer** (optional) — or leave as Walk-in Customer
5. **Apply overall discount** — enter peso amount
6. **Select payment method** — Cash, Card, GCash, Bank Transfer, Store Credit
7. **Enter amount paid** — change is automatically computed
8. **Click ✓ Complete Sale**

### VAT Computation

- VAT rate is configured in Settings (default: **12%**)
- VAT is applied per item to the discounted subtotal
- VAT-exempt products are excluded from VAT computation
- The receipt shows: Subtotal → VAT → Total

### After Sale

- A success dialog shows the sale number, total, change, and payment method
- An Official Receipt is automatically generated
- Click **Print Receipt** to print or **New Sale** to start the next transaction

---

## 6. Sales History

**Path:** Sales → Sales History  
**Access:** Cashier (own sales only), Manager+ (all sales)

### Filters

| Filter | Options |
|--------|---------|
| Search | Sale number or customer name |
| Date Range | From and To date |
| Payment Method | Cash, Card, GCash, Bank Transfer, Store Credit |
| Status | Completed, Void |

### Viewing a Sale

Click any row to open the full sale detail including all line items, totals, and payment information.

### Voiding a Sale

**Access:** Manager, Admin, Super Admin

1. Click **Void** on a completed sale row
2. Enter a mandatory void reason
3. Click **Void Sale**

Effect: Sale status → Void, stock quantities are restored, linked OR is voided.

---

## 7. Invoices / Official Receipts

**Path:** Invoices / OR  
**Access:** Cashier, Manager, Admin, Super Admin

Every completed sale automatically generates an Official Receipt. The OR number follows the format:

```
{PREFIX}-{YYYYMMDD}-{SEQUENCE}
Example: OR-20240415-000001
```

### Printing an Official Receipt

1. Find the invoice using filters
2. Click **View / Print**
3. Review the receipt details
4. Click **🖨️ Print**

The receipt includes:
- Company name, address, TIN, and phone
- OR number and date
- Customer name, address, and TIN (if recorded)
- Itemised product list with quantities and prices
- VATable sales subtotal
- Output VAT (12%)
- Total Amount Due
- Receipt footer message

---

## 8. Customers

**Path:** Customers  
**Access:** View — All roles | Add/Edit — Cashier+

### Customer Fields

| Field | BIR Relevance |
|-------|--------------|
| Full Name | Appears on OR |
| Phone | Contact only |
| Email | Contact only |
| TIN Number | Required for VAT invoice to businesses |
| Address | Appears on OR |

**TIN Format:** 000-000-000-000

---

## 9. Suppliers

**Path:** Suppliers  
**Access:** Manager, Admin, Super Admin

Record supplier information for purchase tracking:
- Company name, contact person, phone, email
- Address, TIN number

---

## 10. Reports

All reports support date range filtering and can be printed or exported to PDF.

### 10.1 Sales Report

**Path:** Reports → Sales Report  
**Access:** Manager, Admin, Super Admin

Shows all transactions in the selected date range with:
- Individual transaction details (sale#, date, cashier, customer, amounts)
- Summary totals: transaction count, subtotal, discounts, VAT, grand total

### 10.2 Inventory Report

**Path:** Reports → Inventory Report  
**Access:** Manager, Admin, Super Admin

Real-time inventory snapshot showing:
- All active products with current quantities
- Cost price, selling price, inventory value per product
- Stock status (In Stock / Low Stock / Out of Stock)
- Summary: total items, total inventory value, low stock count

### 10.3 Profit & Loss Report

**Path:** Reports → Profit & Loss  
**Access:** Admin, Super Admin

Financial performance for the selected period:
- Total Revenue (net of discounts)
- Cost of Goods Sold (based on product cost prices)
- Gross Profit and Gross Margin %
- VAT collected and discounts given
- Breakdown by product category

### 10.4 VAT Report (BIR)

**Path:** Reports → VAT Report (BIR)  
**Access:** Admin, Super Admin

Formatted for BIR output tax filing:
- Lists every OR in the period with customer TIN
- Summary: VATable Sales, Output VAT (12%)
- Suitable for preparing BIR Form 2550M (monthly) and 2550Q (quarterly)

---

## 11. User Management

**Path:** User Management  
**Access:** Admin (view/edit), Super Admin (full control)

### User Roles

| Role | Description |
|------|-------------|
| Super Admin | Full system access including settings and user management |
| Admin | All operations except system settings |
| Manager | Inventory, sales, all reports, no user management |
| Cashier | POS only, view own sales and invoices |
| Viewer | Read-only access to products and dashboard |

### Creating a User

1. Click **+ Add User**
2. Enter: Username (unique), Full Name, Email (optional), Role, Password (min. 6 characters)
3. Click **Save User**

### Changing Passwords

- **Admins** can reset any user's password via the Edit user form
- **All users** can change their own password in the Change My Password section

### Deactivating Users

Click **Deactivate** — the user cannot log in but their historical data is preserved.

---

## 12. System Settings

**Path:** Settings  
**Access:** Super Admin only

| Setting | Description |
|---------|-------------|
| Company Name | Printed on all receipts and reports |
| Address | Printed on official receipts |
| Phone | Displayed on receipts |
| Email | Company contact |
| TIN Number | BIR Tax Identification Number |
| VAT Rate | Default: 12% |
| Currency Symbol | Default: ₱ |
| Invoice/OR Prefix | Default: OR |
| Sale Number Prefix | Default: SL |
| Low Stock Threshold | Default: 10 |
| Receipt Footer | Message printed at the bottom of receipts |

---

## 13. Access Control Reference

| Feature | Viewer | Cashier | Manager | Admin | Super Admin |
|---------|:------:|:-------:|:-------:|:-----:|:-----------:|
| Dashboard | ✓ | ✓ | ✓ | ✓ | ✓ |
| View Products | ✓ | ✓ | ✓ | ✓ | ✓ |
| Add/Edit Products | | | ✓ | ✓ | ✓ |
| Delete Products | | | | ✓ | ✓ |
| Manage Categories | | | ✓ | ✓ | ✓ |
| Stock Adjustments | | | ✓ | ✓ | ✓ |
| Point of Sale | | ✓ | ✓ | ✓ | ✓ |
| View All Sales | | | ✓ | ✓ | ✓ |
| View Own Sales | | ✓ | ✓ | ✓ | ✓ |
| Void Sales | | | ✓ | ✓ | ✓ |
| View Invoices | | ✓ | ✓ | ✓ | ✓ |
| Manage Customers | | ✓ | ✓ | ✓ | ✓ |
| Manage Suppliers | | | ✓ | ✓ | ✓ |
| Sales Report | | | ✓ | ✓ | ✓ |
| Inventory Report | | | ✓ | ✓ | ✓ |
| Profit & Loss | | | | ✓ | ✓ |
| VAT Report | | | | ✓ | ✓ |
| User Management | | | | ✓ | ✓ |
| System Settings | | | | | ✓ |

---

*LumiSync v1.0 — Built for Philippine Electronics & Lighting Businesses*
