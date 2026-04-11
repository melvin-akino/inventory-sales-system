# LumiSync — Quick Start Guide

Get up and running in under 10 minutes.

---

## Step 1 — Install

```bash
git clone <repository-url>
cd inventory-sales-system
chmod +x setup.sh
./setup.sh
```

> Rust and Node.js are auto-installed if missing. On first Rust build, expect 3–10 minutes.

---

## Step 2 — First Login

| Field | Value |
|-------|-------|
| Username | `admin` |
| Password | `Admin@123` |

⚠️ **Change this password now** → User Management → Change My Password

---

## Step 3 — Company Setup

Go to **⚙️ Settings** and fill in:

- Company name
- Address
- TIN number (BIR)
- OR prefix (default: `OR`)

---

## Step 4 — Add Products

Go to **📦 Products → + Add Product**

Minimum required fields:
- Name, SKU, Cost Price, Selling Price, Initial Stock

---

## Step 5 — Make Your First Sale

1. Go to **🛒 Sales → Point of Sale**
2. Search and click products to add to cart
3. Enter amount paid
4. Click **✓ Complete Sale**

An Official Receipt is automatically generated!

---

## Step 6 — Add Staff Accounts

Go to **👤 User Management → + Add User**

| Role | Best for |
|------|----------|
| Cashier | Counter staff — POS only |
| Manager | Supervisors — inventory + reports |
| Admin | Accountants — all reports + users |

---

## Quick Reference

| Action | Path |
|--------|------|
| New sale | Sales → Point of Sale |
| Check stock | Products → filter Low Stock |
| Print OR | Invoices / OR → View / Print |
| Daily sales | Reports → Sales Report |
| VAT filing | Reports → VAT Report (BIR) |
| Receive stock | Products → Stock button |
| Add user | User Management → + Add User |

---

## Default Login Reminder

```
admin / Admin@123
```

*Change immediately after first login.*
