# 📚 LumiSync System - Complete User Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Login & Company Branding](#login--company-branding)
3. [Point of Sale (POS)](#point-of-sale-pos)
4. [Inventory Management](#inventory-management)
5. [Sales & Transactions](#sales--transactions)
6. [Customers](#customers)
7. [Reports](#reports)
8. [Settings](#settings)
9. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Default Login Credentials
- **Username:** admin
- **Password:** Admin@123

⚠️ **Important:** Change this password immediately after first login for security.

### First Login Steps
1. Go to http://localhost:8080 in your web browser
2. Enter username: `admin`
3. Enter password: `Admin@123`
4. Click "Sign In"
5. You'll be directed to the Dashboard

### Changing Your Password
1. Click your username in the top-right corner
2. Select "Settings"
3. Go to "Profile" section
4. Enter current password and new password
5. Click "Update Password"

---

## Login & Company Branding

### Dynamic Company Branding
The login page automatically displays your company information:
- **Company Name** - Updated from Settings
- **Company Address/Location** - Updated from Settings
- **Company Logo** - If configured in Settings
- **Company Subtitle** - Your business tagline

### Updating Company Information
1. Log in to the system
2. Go to **Settings** (⚙️ icon in top right)
3. Update the following fields:
   - Company Name
   - Company Address
   - Company Phone
   - Company Email
   - Company TIN/Tax Number
   - Company Subtitle

4. Click "Save Settings"
5. Log out and log back in to see changes on the login page

### Adding Company Logo
1. Go to **Settings**
2. In Company Settings, upload your company logo
3. Logo will appear on the login page and throughout the system

---

## Point of Sale (POS)

### Accessing POS
1. Click **Sales** in the main menu
2. Click **Point of Sale**
3. Or directly visit: `/#/sales/pos`

### POS Layout

The screen is divided into three sections:

#### Left Panel - Categories & Search
- **Search Bar:** Find products by name or SKU
- **Category List:** Filter products by category
- **Quick Actions:** Clear cart, add customer

#### Center Panel - Product Grid
- **5-Column Grid:** Browse all available products
- **Product Cards:** Show name, SKU, price, stock status
- **Click to Add:** Click any product to add to cart

#### Right Panel - Shopping Cart
- **Cart Items List:** Shows all selected products with quantities
- **Quantity Controls:** Adjust items with [−] and [+]
- **Payment Section:** Discount, VAT, totals
- **Payment Method:** Select Cash, Card, Check, or Mixed
- **Amount Paid:** Enter payment amount
- **Change Display:** Automatically calculated

### How to Process a Sale

1. **Browse Products**
   - See all products in the grid
   - Use search bar to find specific items
   - Click category filters to narrow down

2. **Add Items to Cart**
   - Click any product to add it
   - Click again to add more units
   - Each click adds 1 unit (max = available stock)

3. **Manage Cart Items**
   - Use [−] button to decrease quantity
   - Use [+] button to increase quantity
   - Type quantity directly in the field
   - Click [✕] to remove item completely

4. **Apply Discounts** (Optional)
   - Enter discount percentage (0-100%)
   - OR enter fixed discount amount in ₱
   - Totals update automatically

5. **Select Payment Method**
   - Cash 💵
   - Card 💳
   - Check ✓
   - Mixed payment

6. **Enter Amount Paid**
   - Type the amount customer gave
   - Change is calculated automatically in green box

7. **Complete Sale**
   - Review totals: Subtotal, Discount, VAT, TOTAL
   - Click "✓ Complete Sale" button
   - Button is disabled if amount is insufficient
   - Success message appears when sale is saved

8. **Print Receipt** (Optional)
   - Click "🖨️ Print" to get receipt preview
   - Print from the browser print dialog

### Tips for Fast POS Operations
- Use product search to quickly find items
- Click products multiple times to add quantities
- Press Tab key to move between fields
- Use number pad for faster input
- Remember most popular SKUs for quick lookup

---

## Inventory Management

### Accessing Inventory
Click **Inventory** in the main menu to see options:
- Products
- Categories
- Stock Adjustments

### Managing Products

#### View All Products
1. Click **Inventory** → **Products**
2. See complete product list with:
   - Product name and SKU
   - Category
   - Cost price and selling price
   - Current stock quantity
   - Stock status (In Stock/Low Stock/Out of Stock)

#### Add New Product
1. Click **Inventory** → **Products**
2. Click **"+ Add Product"** button
3. Fill in details:
   - **Product Name** (required)
   - **SKU** (unique product code, required)
   - **Category** (select from list)
   - **Description** (optional)
   - **Unit** (piece, box, roll, etc.)
   - **Cost Price** (what you paid for it)
   - **Selling Price** (what customers pay)
   - **Quantity** (current stock)
   - **Reorder Level** (alert when stock falls below this)

4. Click **"Save Product"**

#### Edit Product
1. Click on a product in the list
2. Modify any details
3. Click **"Update Product"**

#### Delete Product
1. Click on a product
2. Click **"Delete Product"**
3. Confirm deletion

### Managing Categories

#### View Categories
1. Click **Inventory** → **Categories**
2. See all product categories

#### Add Category
1. Click **"+ Add Category"**
2. Enter category name and description
3. Click **"Create Category"**

#### Edit Category
1. Click on a category
2. Update name or description
3. Click **"Update Category"**

### Stock Adjustments

#### Why Adjust Stock
- Items received from supplier don't match records
- Items damaged or lost
- Physical inventory count reveals discrepancies
- Manual correction needed

#### Make Stock Adjustment
1. Click **Inventory** → **Stock Adjustments**
2. Click **"+ Add Adjustment"**
3. Select product
4. Choose adjustment type:
   - **Add:** Increase stock (new delivery, finds)
   - **Subtract:** Decrease stock (damage, loss)
   - **Set:** Set exact quantity (after physical count)
5. Enter quantity change
6. Add reason/notes
7. Click **"Save Adjustment"**

#### View Adjustment History
All adjustments are logged and can be viewed with:
- Product name
- Date and time
- Quantity before and after
- Reason for adjustment

---

## Sales & Transactions

### Accessing Sales

#### Point of Sale
For quick checkout: **Sales** → **Point of Sale**

#### Sales History
View past sales: **Sales** → **Sales History**

### Sales History Features

#### View Sales Records
1. Click **Sales** → **Sales History**
2. See all completed sales with:
   - Sale number
   - Date and time
   - Customer name (if recorded)
   - Total amount
   - Payment method
   - Status

#### Search Sales
- Use date range filter to find sales by date
- Search by sale number or customer name
- Filter by payment method

#### View Sale Details
1. Click on a sale record
2. See:
   - All items in the sale
   - Quantities and prices
   - Discounts applied
   - VAT calculation
   - Final total and payment received
   - Change given

#### Void Sales
1. Click on a completed sale
2. If within allowed time, click **"Void Sale"**
3. Enter reason for void
4. Confirm action
5. Sale is marked as voided, stock is restored

### Invoices

#### Official Receipts (OR)
Generate official tax documents:
1. Click **Invoices**
2. View all invoices generated from sales
3. Print official receipt for customer
4. Download PDF for records

---

## Customers

### Customer Management

#### View Customer List
1. Click **Customers**
2. See all registered customers with:
   - Name
   - Phone number
   - Email
   - Address
   - Total purchases

#### Add Customer
1. Click **Customers**
2. Click **"+ Add Customer"**
3. Fill in:
   - Full name
   - Phone number
   - Email address
   - Home/office address
   - TIN number (optional)

4. Click **"Save Customer"**

#### Edit Customer
1. Click on customer name
2. Update information
3. Click **"Update Customer"**

#### Customer History
1. Click on customer name
2. View:
   - All purchases made by customer
   - Total amount spent
   - Last purchase date
   - Purchase frequency

---

## Reports

### Dashboard
**Home** shows overview:
- Daily sales total
- Number of transactions
- Top products sold
- Stock alerts

### Sales Report
**Reports** → **Sales Report**
- Daily, weekly, or monthly sales
- Filter by date range
- View sales trends
- Export data

### Inventory Report
**Reports** → **Inventory Report**
- Current stock levels
- Low stock alerts
- Top products by quantity
- Stock value

### Profit & Loss
**Reports** → **Profit & Loss**
- Revenue vs. cost
- Profit margin
- Period comparison
- Product profitability

### VAT Report (BIR)
**Reports** → **VAT Report**
- VAT collected summary
- Required for tax filing
- Monthly/quarterly reports
- Export for BIR submission

---

## Settings

### Company Settings
**Settings** → **Company**
- Company name
- Address and contact info
- Tax number (TIN)
- Company logo upload
- Subtitle/tagline

### System Settings
**Settings** → **System**
- VAT rate (default 12%)
- Currency symbol (₱)
- Receipt footer message
- Low stock threshold
- Invoice prefix

### User Management
**Settings** → **Users**

#### View Users
See all system users with their roles

#### Add User
1. Click **"+ Add User"**
2. Enter:
   - Full name
   - Username (for login)
   - Email
   - Password
   - Role (Admin, Manager, Cashier, Viewer)

3. Click **"Create User"**

#### Edit User
1. Click on user
2. Update information
3. Click **"Update User"**

#### Reset User Password
1. Click on user
2. Click **"Reset Password"**
3. Temporary password is created
4. User must change on next login

#### Deactivate User
1. Click on user
2. Click **"Deactivate"**
3. User cannot log in but records remain

### User Roles

#### Super Admin
- Full system access
- Manage all users and settings
- View all reports

#### Admin
- Create and manage products
- Manage categories
- View all reports
- Cannot delete users

#### Manager
- Process sales (POS)
- View inventory
- Manage stock adjustments
- Cannot manage users or system settings

#### Cashier
- Process sales only
- Cannot modify products or inventory
- View own sales records

#### Viewer
- Read-only access
- Cannot make any changes
- View reports only

---

## Troubleshooting

### Login Issues

**Problem:** Can't log in / "Invalid credentials"
- Check username spelling
- Confirm caps lock is off
- Verify password is correct
- Default: admin / Admin@123
- Contact admin if stuck

**Problem:** Forgot password
- Click "Need help?" on login page
- Contact administrator for password reset
- Admin can reset your password from Settings

### POS Issues

**Problem:** Products not showing in POS
- Check products have been created in Inventory
- Verify products are marked as "Active"
- Confirm products have a selling price
- Search bar won't find products without these

**Problem:** Can't add items to cart
- Refresh page (Ctrl+F5)
- Check backend is running (should see API messages)
- Verify product has stock available
- Click product once more

**Problem:** Discount not applying
- Enter as percentage (0-100) for discount %
- OR enter fixed amount in ₱ for discount ₱
- Check VAT is calculated (should be on subtotal after discount)
- Totals should update when you enter discount

**Problem:** Sale won't complete
- Ensure amount paid ≥ total
- "Complete Sale" button should be green (enabled)
- Check cart has at least 1 item
- Review error message shown

### Inventory Issues

**Problem:** Stock levels incorrect
- Use Stock Adjustments to correct
- Perform physical count
- Mark adjustment reason in notes
- Check adjustment history

**Problem:** Low stock alerts not working
- Verify reorder level is set (Inventory → Products)
- Reorder level must be > 0 and < current quantity
- Dashboard and reports should show items below reorder level

### Reporting Issues

**Problem:** Sales not showing in reports
- Ensure sales were completed (status = "completed")
- Check date range includes sale date
- Voided sales show separately
- Refresh report or clear browser cache

**Problem:** VAT calculation incorrect
- Check VAT rate in Settings (default 12%)
- VAT applies to subtotal AFTER discount
- Some items may be VAT exempt (check product settings)

### General Issues

**Problem:** System running slow
- Close unnecessary browser tabs
- Clear browser cache (Ctrl+Shift+R)
- Check internet connection
- Restart server if local

**Problem:** Data not saving
- Check internet connection
- Verify backend server is running
- Try again after 10 seconds
- Check browser console for errors (F12)

**Problem:** Need to clear browser cache
- Windows/Linux: Press **Ctrl + Shift + R**
- Mac: Press **Cmd + Shift + R**
- Or: F12 → Application → Clear Site Data

---

## Help & Support

### Help Page
- Click **"Need help?"** on login page
- Or navigate to **/#/help**
- Complete user guide with examples
- Feature overview and tips

### Getting More Help
- Contact your system administrator
- Check Dashboard for system status
- Review error messages carefully
- Try clearing cache and refreshing

### System Information
- Version: 1.0.0
- Backend: Rust/Axum API
- Database: SQLite
- Frontend: Vue 3 + Vite

---

## Quick Reference

### Key Keyboard Shortcuts
- **Ctrl + Shift + R:** Hard refresh (clear cache)
- **Tab:** Move between fields
- **Enter:** Submit form
- **Esc:** Close modal/dialog

### Key Metrics to Track
- Daily sales total
- Items sold per day
- Best-selling products
- Stock turnover rate
- Profit margin per product

### Common Scenarios

**Scenario 1: Customer buys 5 LED bulbs**
1. Go to POS
2. Search "LED"
3. Click LED Bulb product 5 times
4. See quantity = 5 in cart
5. Payment method = Cash
6. Enter amount paid
7. Complete sale

**Scenario 2: New product delivery arrives**
1. Go to Inventory → Products
2. Find product
3. Click to edit
4. Update quantity
5. OR use Stock Adjustments with reason "Supplier delivery"

**Scenario 3: End of day reporting**
1. Go to Dashboard or Reports
2. Check daily sales total
3. Note best sellers
4. Check low stock items
5. Plan next day's inventory needs

---

## Best Practices

✓ Update company information for professional branding
✓ Use unique SKUs for all products
✓ Keep prices current and accurate
✓ Set appropriate reorder levels
✓ Record all sales through POS (not manual)
✓ Review inventory regularly
✓ Reconcile cash with system records
✓ Create customer records for loyalty tracking
✓ Generate weekly sales reports
✓ Backup system regularly

---

## Contact & Support

For technical issues or feature requests:
- Contact your system administrator
- Check help page for solutions
- Review system logs (Administrator access required)

---

**Last Updated:** April 2026
**System Version:** 1.0.0
**For:** LumiSync Inventory & Sales System
