// ─────────────────────────────────────────────────────────────────────────────
//  Help Centre content — articles, categories, search index
// ─────────────────────────────────────────────────────────────────────────────

export const HELP_ARTICLES = [
  // ── Getting Started ────────────────────────────────────────────────────────
  {
    id: 'getting-started',
    category: 'Getting Started',
    icon: '🚀',
    title: 'Getting Started with LumiSync',
    summary: 'First steps: logging in, navigating the system, and initial setup.',
    tags: ['start', 'login', 'setup', 'first', 'begin', 'intro'],
    content: `
## Welcome to LumiSync

LumiSync is an all-in-one inventory, sales, and invoicing system built for Philippine electronics and lighting businesses.

---

## First Login

1. Open LumiSync (desktop app or browser).
2. Enter the default credentials:
   - **Username:** \`admin\`
   - **Password:** \`Admin@123\`
3. Click **Sign In**.

> ⚠️ Change the default password immediately via **User Management → Change My Password**.

---

## Navigating the System

The sidebar on the left contains all major sections:

| Section | What it does |
|---------|-------------|
| 📊 Dashboard | Sales summary, top products, recent transactions |
| 📦 Products | Full product catalogue with stock levels |
| 🛒 Point of Sale | Create new sales transactions |
| 🧾 Invoices / OR | Official Receipts and invoice history |
| 👥 Customers | Customer records and TIN management |
| 📈 Reports | Sales, Inventory, Profit & Loss, VAT |
| 👤 Users | Manage system users and roles |
| ⚙️ Settings | Company info, VAT rate, OR prefix |

---

## Initial Setup Checklist

- [ ] Change the admin password
- [ ] Set your company name, address, and TIN in **Settings**
- [ ] Configure the VAT rate (default 12%)
- [ ] Set OR / Invoice number prefix
- [ ] Add product categories
- [ ] Import your product catalogue
- [ ] Create additional user accounts
    `,
  },

  // ── POS ───────────────────────────────────────────────────────────────────
  {
    id: 'pos-guide',
    category: 'Sales',
    icon: '🛒',
    title: 'Making a Sale (Point of Sale)',
    summary: 'Step-by-step guide to processing a sales transaction at the POS screen.',
    tags: ['sale', 'pos', 'cash', 'payment', 'cart', 'checkout', 'transaction'],
    content: `
## Making a Sale

Navigate to **Sales → Point of Sale**.

---

## Step-by-Step

### 1. Search for Products
- Type in the search bar (top-left) to find products by name or SKU.
- Filter by category using the dropdown.
- Click a product card to add it to the cart.

### 2. Manage the Cart
- **Quantity:** Use the **+** and **−** buttons to adjust quantity.
- **Discount:** Enter a per-item discount percentage (e.g., 10 for 10% off).
- **Remove:** Click **✕** to remove an item from the cart.
- **Clear:** Click **Clear** to empty the entire cart.

### 3. Select a Customer (Optional)
- Choose an existing customer from the dropdown for named receipt.
- Leave as **Walk-in Customer** for anonymous cash sales.

### 4. Apply Overall Discount
- Enter a peso amount in the **Discount (₱)** field to deduct from the total.

### 5. Select Payment Method
| Method | Use when |
|--------|----------|
| Cash | Physical cash payment |
| Card | Credit or debit card |
| GCash | GCash mobile wallet |
| Bank Transfer | Wire or online banking |
| Store Credit | Credit account for regular customers |

### 6. Enter Amount Paid
- Type the amount received from the customer.
- The **change** is automatically computed.

### 7. Complete the Sale
- Click **✓ Complete Sale**.
- An Official Receipt (OR) is automatically generated.
- A receipt summary is shown — click **Print Receipt** to print.

---

## Tips
- Products with **0 stock** are greyed out and cannot be added to cart.
- The system prevents payment if the amount paid is less than the total.
- Sales are automatically linked to an Official Receipt number.
    `,
  },

  // ── Void Sale ─────────────────────────────────────────────────────────────
  {
    id: 'void-sale',
    category: 'Sales',
    icon: '↩️',
    title: 'Voiding a Sale',
    summary: 'How to cancel a completed sale and restore inventory.',
    tags: ['void', 'cancel', 'reverse', 'return', 'refund'],
    content: `
## Voiding a Sale

> 🔒 **Access required:** Manager, Admin, or Super Admin

Navigate to **Sales → Sales History**.

---

## Steps

1. Locate the sale in the list (use filters to narrow down).
2. Click on the row to view the sale detail, or click the **Void** button directly.
3. Enter a **reason** for voiding (required).
4. Click **Void Sale**.

---

## What Happens When a Sale is Voided

- The sale status changes from **Completed** to **Void**.
- All stock quantities are **automatically restored** for each item.
- The linked Official Receipt is also marked as **Void**.
- The void reason is recorded in the sale notes.

---

## Important Notes

- Only **completed** sales can be voided.
- Already voided sales cannot be re-voided.
- Voided sales remain in the history for audit purposes.
- Cashiers cannot void sales — a Manager or higher is required.
    `,
  },

  // ── Invoices ──────────────────────────────────────────────────────────────
  {
    id: 'invoice-print',
    category: 'Invoices',
    icon: '🖨️',
    title: 'Viewing and Printing Official Receipts',
    summary: 'How to find, view, and print invoices or official receipts.',
    tags: ['invoice', 'receipt', 'print', 'OR', 'official receipt', 'bir'],
    content: `
## Official Receipts (OR)

Every completed sale automatically generates an Official Receipt with a unique OR number.

Navigate to **Invoices / OR** in the sidebar.

---

## Finding an Invoice

Use the filters at the top:
- **Search:** Enter OR number, sale number, or customer name.
- **Date Range:** Filter by invoice date.
- **Status:** Filter by Paid or Void.

---

## Viewing an Invoice

Click **View / Print** on any row to open the invoice detail.

The invoice displays:
- Company name, address, and TIN
- OR number and date
- Customer name, address, and TIN (if provided)
- Itemised list of products sold
- VATable sales amount
- VAT (12%) breakdown
- Total amount due
- Footer message

---

## Printing

1. Open the invoice by clicking **View / Print**.
2. Click the **🖨️ Print** button.
3. Your system print dialog will open.
4. Select your printer or **Save as PDF**.

---

## OR Number Format

OR numbers follow the format configured in Settings:

\`\`\`
{PREFIX}-{YYYYMMDD}-{SEQUENCE}
Example: OR-20240415-000001
\`\`\`

You can change the prefix in **Settings → Invoice/OR Prefix**.
    `,
  },

  // ── Products ──────────────────────────────────────────────────────────────
  {
    id: 'add-product',
    category: 'Inventory',
    icon: '📦',
    title: 'Adding and Managing Products',
    summary: 'How to add, edit, and deactivate products in the catalogue.',
    tags: ['product', 'add', 'sku', 'price', 'stock', 'inventory', 'catalogue', 'item'],
    content: `
## Managing Products

Navigate to **Products** in the sidebar.

> 🔒 **Adding/Editing:** Manager, Admin, or Super Admin only.
> **Viewing:** All users.

---

## Adding a Product

1. Click **+ Add Product**.
2. Fill in the required fields:

| Field | Description |
|-------|-------------|
| Product Name | Full product name (e.g., *LED Bulb 9W Daylight*) |
| SKU | Unique stock-keeping unit code (e.g., *LED-9W-DL*) |
| Category | Select from existing categories |
| Unit | piece, box, set, meter, roll |
| Cost Price | Your purchase cost (for P&L reports) |
| Selling Price | Price charged to customers |
| Initial Stock | Starting stock quantity |
| Reorder Level | Threshold for low-stock alert |
| VAT Exempt | Check if product is not subject to 12% VAT |

3. Click **Save Product**.

---

## Editing a Product

1. Find the product in the list (use search or category filter).
2. Click **Edit**.
3. Modify any fields and click **Save Product**.

> Note: The SKU cannot be changed once a product has sales history.

---

## Product Status Indicators

| Badge | Meaning |
|-------|---------|
| 🟢 In Stock | Quantity is above the reorder level |
| 🟡 Low Stock | Quantity is at or below reorder level |
| 🔴 Out of Stock | Quantity is zero |

---

## Deactivating a Product

Click **Delete** → the product is deactivated (not permanently removed) and hidden from the POS.
    `,
  },

  // ── Stock Adjustment ──────────────────────────────────────────────────────
  {
    id: 'stock-adjustment',
    category: 'Inventory',
    icon: '⚖️',
    title: 'Adjusting Stock Levels',
    summary: 'Manually add, remove, or set stock quantities with full audit trail.',
    tags: ['stock', 'adjust', 'quantity', 'inventory', 'add stock', 'receive', 'audit'],
    content: `
## Stock Adjustments

Use stock adjustments to update quantities when receiving deliveries, performing stock counts, or writing off damaged goods.

> 🔒 **Access required:** Manager, Admin, or Super Admin

---

## How to Adjust Stock

1. Go to **Products**.
2. Find the product and click **Stock**.
3. Choose an adjustment type:

| Type | Description |
|------|-------------|
| **Add** | Increase stock (e.g., received delivery) |
| **Subtract** | Decrease stock (e.g., damaged, written off) |
| **Set** | Set to an exact count (e.g., after physical stocktake) |

4. Enter the quantity and an optional reason.
5. Click **Apply**.

---

## Viewing Adjustment History

Navigate to **Inventory → Stock Adjustments** to see the full audit log of all adjustments including:
- Date and time
- Product name
- Before / change / after quantities
- Reason
- User who made the adjustment

---

## Best Practices

- Always enter a reason (e.g., *"Received from Supplier — PO#2024-001"*).
- Use **Set** type after a full physical stock count to reconcile.
- Review the adjustment log regularly to spot discrepancies.
    `,
  },

  // ── Categories ────────────────────────────────────────────────────────────
  {
    id: 'categories',
    category: 'Inventory',
    icon: '🗂️',
    title: 'Managing Product Categories',
    summary: 'Create and edit categories to organise your product catalogue.',
    tags: ['category', 'categories', 'group', 'organise', 'organize'],
    content: `
## Product Categories

Categories help organise your product catalogue and are used for filtering in the POS and reporting.

Navigate to **Inventory → Categories**.

---

## Default Categories

LumiSync comes pre-loaded with categories for a lighting business:

- LED Bulbs
- Fluorescent
- Downlights
- Streetlights
- Floodlights
- Decorative
- Accessories

---

## Adding a Category

1. Click **+ Add Category**.
2. Enter the category name and an optional description.
3. Click **Save**.

---

## Editing a Category

1. Find the category in the list.
2. Click **Edit**.
3. Update the name or description.
4. Click **Save**.

> Categories cannot be deleted if products are assigned to them. Reassign products first.
    `,
  },

  // ── Customers ─────────────────────────────────────────────────────────────
  {
    id: 'customers',
    category: 'CRM',
    icon: '👥',
    title: 'Managing Customers',
    summary: 'Add and manage customer records including TIN for official receipts.',
    tags: ['customer', 'client', 'tin', 'vat', 'address', 'crm'],
    content: `
## Customer Management

Navigate to **Customers** in the sidebar.

---

## Why Add Customers?

- Generate official receipts with the customer's name and TIN
- Track sales history per customer
- Required for VAT invoices with TIN for B2B sales

---

## Adding a Customer

1. Click **+ Add Customer**.
2. Fill in the fields:

| Field | Notes |
|-------|-------|
| Full Name | Required |
| Phone | Optional |
| Email | Optional |
| TIN Number | Format: 000-000-000-000 (for BIR compliance) |
| Address | Appears on the official receipt |

3. Click **Save**.

---

## Selecting a Customer at POS

At the Point of Sale screen, use the **Customer** dropdown to select a registered customer before completing a sale.

---

## Walk-in Sales

If no customer is selected, the sale and receipt will show **Walk-in Customer**.
    `,
  },

  // ── Users & Roles ─────────────────────────────────────────────────────────
  {
    id: 'user-roles',
    category: 'Administration',
    icon: '👤',
    title: 'User Roles and Access Control',
    summary: 'Understand what each role can access and how to manage user accounts.',
    tags: ['user', 'role', 'access', 'permission', 'admin', 'cashier', 'manager', 'login', 'account'],
    content: `
## User Roles

LumiSync uses role-based access control. Each user is assigned one of five roles.

---

## Role Permissions

| Feature | Viewer | Cashier | Manager | Admin | Super Admin |
|---------|--------|---------|---------|-------|-------------|
| View Products | ✓ | ✓ | ✓ | ✓ | ✓ |
| Add/Edit Products | | | ✓ | ✓ | ✓ |
| Point of Sale | | ✓ | ✓ | ✓ | ✓ |
| View Sales History | Own | ✓ | ✓ | ✓ | ✓ |
| Void Sales | | | ✓ | ✓ | ✓ |
| View Invoices | | ✓ | ✓ | ✓ | ✓ |
| Sales Reports | | | ✓ | ✓ | ✓ |
| Inventory Report | | | ✓ | ✓ | ✓ |
| Profit & Loss | | | | ✓ | ✓ |
| VAT Report | | | | ✓ | ✓ |
| User Management | | | | ✓ | ✓ |
| System Settings | | | | | ✓ |

---

## Creating a User

> 🔒 Super Admin only

1. Go to **User Management**.
2. Click **+ Add User**.
3. Enter username, full name, role, and password (min. 6 characters).
4. Click **Save User**.

---

## Changing a Password

- **Admin changing another user's password:** Edit the user and enter a new password.
- **Any user changing their own password:** Scroll down on the User Management page to the **Change My Password** section.

---

## Deactivating a User

Click **Deactivate** next to a user. They will no longer be able to log in. Their historical data is preserved.
    `,
  },

  // ── Reports Overview ──────────────────────────────────────────────────────
  {
    id: 'reports-overview',
    category: 'Reports',
    icon: '📊',
    title: 'Reports Overview',
    summary: 'Summary of all available reports and how to use them.',
    tags: ['report', 'reports', 'analytics', 'summary', 'overview', 'data'],
    content: `
## Available Reports

LumiSync provides four key accounting reports. All reports support date range filtering and printing.

---

### 📈 Sales Report
**Access:** Manager, Admin, Super Admin
**Path:** Reports → Sales Report

Shows all transactions in a date range with:
- Sale number, date, cashier, customer
- Subtotal, discount, VAT, and total
- Summary totals: transactions, revenue, VAT collected

---

### 📉 Inventory Report
**Access:** Manager, Admin, Super Admin
**Path:** Reports → Inventory Report

Real-time snapshot of all active products:
- Current stock levels vs. reorder levels
- Cost price, selling price, inventory value
- Low Stock / Out of Stock flags
- Total inventory valuation

---

### 💹 Profit & Loss Report
**Access:** Admin, Super Admin
**Path:** Reports → Profit & Loss

Financial performance for any date range:
- Total revenue (subtotals)
- Cost of goods sold (based on cost price)
- Gross profit and gross margin %
- VAT collected
- Discounts given
- Breakdown by product category

---

### 🏛️ VAT Report (BIR)
**Access:** Admin, Super Admin
**Path:** Reports → VAT Report (BIR)

Formatted for Philippine BIR output tax filing:
- VATable sales total
- Output VAT (12%)
- Per-OR breakdown with customer TIN
- Suitable for BIR Form 2550M/Q preparation

---

## Printing Reports

Every report has a 🖨️ **Print** button that opens your system print dialog. Use **Save as PDF** to export.
    `,
  },

  // ── VAT Report ────────────────────────────────────────────────────────────
  {
    id: 'vat-report',
    category: 'Reports',
    icon: '🏛️',
    title: 'VAT Report and BIR Compliance',
    summary: 'How to generate the VAT output report for BIR filing in the Philippines.',
    tags: ['vat', 'bir', 'tax', 'output tax', '2550', 'quarterly', 'monthly', 'philippines'],
    content: `
## VAT Report (BIR Output Tax)

The VAT Report helps you prepare your monthly and quarterly VAT returns for the Bureau of Internal Revenue (BIR).

Navigate to **Reports → VAT Report (BIR)**.

---

## Generating the Report

1. Set the **From** and **To** dates (e.g., first to last day of the month).
2. Click **Generate Report**.

---

## What the Report Shows

| Column | Description |
|--------|-------------|
| OR Number | Official Receipt number |
| Date | Transaction date |
| Customer | Customer name |
| TIN | Customer Tax Identification Number |
| VATable Amount | Sale subtotal (excluding VAT) |
| VAT Amount | 12% output VAT |
| Total Amount | Grand total |

**Summary box** shows:
- Total VATable Sales
- VAT Exempt Sales
- Total Output VAT

---

## Using with BIR Form 2550M / 2550Q

The totals from this report map directly to:
- **Line 1:** VATable Sales → Total VATable Sales
- **Line 6:** Output Tax → Output VAT total

---

## VAT-Exempt Products

Mark products as **VAT Exempt** when creating or editing them. These items will not contribute to VAT output.

---

## Tips for BIR Compliance

- Ensure all B2B customers have their **TIN** recorded.
- Collect OR numbers sequentially — the system auto-increments.
- Keep the OR prefix consistent (set in **Settings**).
- Back up your database monthly before filing.
    `,
  },

  // ── Settings ──────────────────────────────────────────────────────────────
  {
    id: 'settings-guide',
    category: 'Administration',
    icon: '⚙️',
    title: 'System Settings',
    summary: 'Configure company information, VAT rate, OR numbering, and receipt footer.',
    tags: ['settings', 'company', 'tin', 'vat rate', 'prefix', 'configure', 'setup'],
    content: `
## System Settings

> 🔒 **Access required:** Super Admin only

Navigate to **Settings** in the sidebar.

---

## Company Information

| Setting | Description |
|---------|-------------|
| Company Name | Appears on all receipts and reports |
| Address | Printed on official receipts |
| Phone | Displayed on receipts |
| Email | Company contact email |
| TIN Number | Your BIR Tax Identification Number |

---

## Tax & Numbering

| Setting | Default | Description |
|---------|---------|-------------|
| VAT Rate | 12% | Applied to non-VAT-exempt products |
| Currency Symbol | ₱ | Philippine Peso |
| Invoice/OR Prefix | OR | Prefix for Official Receipt numbers |
| Sale Number Prefix | SL | Prefix for sale transaction numbers |
| Low Stock Threshold | 10 | Default reorder alert level |

---

## Receipt Footer

The footer message appears at the bottom of every printed receipt.
Default: *"Thank you for your business!"*

---

## Saving Settings

Click **Save Settings** after making changes. Settings take effect immediately.
    `,
  },

  // ── Troubleshooting ───────────────────────────────────────────────────────
  {
    id: 'troubleshooting',
    category: 'Support',
    icon: '🔧',
    title: 'Troubleshooting Common Issues',
    summary: 'Solutions to the most common problems users encounter.',
    tags: ['error', 'problem', 'fix', 'issue', 'trouble', 'help', 'login', 'stock', 'print'],
    content: `
## Common Issues & Solutions

---

### ❌ Cannot log in

**Cause:** Wrong credentials or deactivated account.

**Solutions:**
- Double-check username and password (case-sensitive).
- Ask a Super Admin to verify your account is active.
- Default credentials: \`admin\` / \`Admin@123\`.

---

### ❌ "Insufficient stock" error at POS

**Cause:** The product quantity in the system is less than what you are trying to sell.

**Solutions:**
- Adjust stock via **Products → Stock** button.
- Verify physical stock matches the system quantity.

---

### ❌ Sale won't complete — amount paid too low

**Cause:** The **Amount Paid** field is less than the total.

**Solution:** Enter the correct amount paid by the customer. Use exact or higher amount.

---

### ❌ Product not appearing in POS

**Cause:** Product is inactive or out of stock.

**Solutions:**
- Go to **Products**, enable "Show Inactive" to find and re-activate the product.
- Adjust stock if the product has 0 quantity.

---

### ❌ Print not working

**Solutions:**
- Click 🖨️ Print — this opens the browser/system print dialog.
- Check that your printer is connected and set as default.
- Use **Save as PDF** to generate a digital copy.

---

### ❌ Access denied error

**Cause:** Your role does not have permission for that action.

**Solution:** Ask a Super Admin or Admin to perform the action, or to upgrade your role.

---

### ❌ OR numbers are out of sequence

**Cause:** Some sales may have been voided.

**Note:** This is normal and acceptable for BIR. Voided ORs should be retained in records.

---

## Contacting Support

If your issue is not listed here, contact your system administrator or the LumiSync support team with:
1. A description of the problem
2. The exact error message (if any)
3. The steps you took before the error occurred
    `,
  },

  // ── Keyboard Shortcuts ────────────────────────────────────────────────────
  {
    id: 'shortcuts',
    category: 'Getting Started',
    icon: '⌨️',
    title: 'Tips & Shortcuts',
    summary: 'Productivity tips to speed up daily operations.',
    tags: ['shortcut', 'tip', 'fast', 'quick', 'keyboard', 'productivity'],
    content: `
## Productivity Tips

---

## POS Tips

- **Quick search:** Just start typing in the POS search bar — no need to click first.
- **Repeat customer:** Select the customer before scanning items so the receipt name is set.
- **Quick total:** After adding items, click the Amount Paid field and type the cash received.
- **Round up:** For cash payments, the system auto-suggests rounding up for easier change.

---

## Inventory Tips

- Use the **Low Stock Only** filter to see items that need reordering.
- Set meaningful **reorder levels** per product (e.g., 5 for slow-movers, 20 for fast-movers).
- Use **SKU codes** consistently (e.g., \`LED-9W-DL\` = LED 9W Daylight) for fast POS searching.

---

## Reporting Tips

- Run the **Sales Report** at end of day (today's date as both From and To).
- Run the **VAT Report** monthly before the 20th for BIR 2550M filing.
- Run the **Inventory Report** monthly to get the inventory valuation for accounting.
- Print the **Profit & Loss** report quarterly for financial review.

---

## Data Safety Tips

- The database is stored locally — back it up regularly by copying \`lumisync.db\`.
- Create separate accounts for each staff member — never share login credentials.
- Only Super Admins should access Settings to prevent accidental misconfiguration.
    `,
  },
]

// ── Categories (with article references) ────────────────────────────────────

export const HELP_CATEGORIES = [
  {
    id: 'getting-started',
    name: 'Getting Started',
    icon: '🚀',
    bg: 'bg-green-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Getting Started'),
  },
  {
    id: 'sales',
    name: 'Sales & POS',
    icon: '🛒',
    bg: 'bg-blue-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Sales'),
  },
  {
    id: 'invoices',
    name: 'Invoices & Receipts',
    icon: '🧾',
    bg: 'bg-yellow-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Invoices'),
  },
  {
    id: 'inventory',
    name: 'Inventory',
    icon: '📦',
    bg: 'bg-purple-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Inventory'),
  },
  {
    id: 'crm',
    name: 'Customers & Suppliers',
    icon: '👥',
    bg: 'bg-indigo-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'CRM'),
  },
  {
    id: 'reports',
    name: 'Reports',
    icon: '📊',
    bg: 'bg-orange-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Reports'),
  },
  {
    id: 'admin',
    name: 'Administration',
    icon: '⚙️',
    bg: 'bg-gray-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Administration'),
  },
  {
    id: 'support',
    name: 'Support',
    icon: '🔧',
    bg: 'bg-red-100',
    articles: HELP_ARTICLES.filter((a) => a.category === 'Support'),
  },
]
