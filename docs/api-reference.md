# LumiSync — Tauri Command API Reference

All commands are invoked via `@tauri-apps/api/tauri` `invoke()` in desktop mode, or via HTTP POST to `/api/{command}` in web mode. Every protected command requires a valid `token` argument.

---

## Authentication

### `login`
```typescript
invoke('login', { request: { username: string, password: string } })
  → { token: string, user: User }
```

### `logout`
```typescript
invoke('logout', { token: string }) → void
```

### `get_current_user`
```typescript
invoke('get_current_user', { token: string }) → User
```

---

## Users

### `get_users`
Roles: `super_admin`, `admin`
```typescript
invoke('get_users', { token }) → User[]
```

### `create_user`
Roles: `super_admin`
```typescript
invoke('create_user', { token, request: {
  username: string, password: string, full_name: string,
  email?: string, role: Role
}}) → User
```

### `update_user`
Roles: `super_admin`
```typescript
invoke('update_user', { token, id: number, request: {
  full_name?: string, email?: string, role?: Role,
  is_active?: boolean, password?: string
}}) → User
```

### `delete_user`
Roles: `super_admin`
```typescript
invoke('delete_user', { token, id: number }) → void
```

### `change_password`
Any authenticated user (own password only)
```typescript
invoke('change_password', { token, currentPassword: string, newPassword: string }) → void
```

---

## Inventory — Categories

### `get_categories`
Roles: All
```typescript
invoke('get_categories', { token }) → Category[]
```

### `create_category`
Roles: `manager+`
```typescript
invoke('create_category', { token, request: { name: string, description?: string }}) → Category
```

### `update_category`
Roles: `manager+`
```typescript
invoke('update_category', { token, id: number, request: { name: string, description?: string }}) → Category
```

---

## Inventory — Products

### `get_products`
Roles: All
```typescript
invoke('get_products', { token, filter?: {
  search?: string, category_id?: number,
  low_stock_only?: boolean, active_only?: boolean
}}) → Product[]
```

### `get_product`
Roles: All
```typescript
invoke('get_product', { token, id: number }) → Product
```

### `create_product`
Roles: `manager+`
```typescript
invoke('create_product', { token, request: {
  category_id?: number, sku: string, name: string,
  description?: string, unit: string, cost_price: number,
  selling_price: number, quantity: number, reorder_level: number,
  is_vat_exempt: boolean
}}) → Product
```

### `update_product`
Roles: `manager+`
```typescript
invoke('update_product', { token, id: number, request: Partial<CreateProductRequest> & { is_active?: boolean }}) → Product
```

### `delete_product`
Roles: `admin+`
```typescript
invoke('delete_product', { token, id: number }) → void
```

### `adjust_stock`
Roles: `manager+`
```typescript
invoke('adjust_stock', { token, request: {
  product_id: number,
  adjustment_type: 'add' | 'subtract' | 'set',
  quantity: number,
  reason?: string
}}) → StockAdjustment
```

### `get_stock_adjustments`
Roles: `manager+`
```typescript
invoke('get_stock_adjustments', { token, productId?: number }) → StockAdjustment[]
```

---

## Sales

### `create_sale`
Roles: `cashier+`
```typescript
invoke('create_sale', { token, request: {
  customer_id?: number,
  items: Array<{ product_id: number, quantity: number, unit_price: number, discount_percent: number }>,
  discount_amount: number,
  amount_paid: number,
  payment_method: 'cash' | 'card' | 'gcash' | 'bank_transfer' | 'credit',
  notes?: string
}}) → Sale
```

### `get_sale`
Roles: All (cashiers limited to own sales)
```typescript
invoke('get_sale', { token, id: number }) → Sale
```

### `get_sales`
Roles: All (cashiers see own sales only)
```typescript
invoke('get_sales', { token, filter?: {
  date_from?: string, date_to?: string, payment_method?: string,
  status?: string, customer_id?: number, search?: string
}}) → Sale[]
```

### `void_sale`
Roles: `manager+`
```typescript
invoke('void_sale', { token, id: number, reason: string }) → Sale
```

---

## Invoices

### `get_invoices`
Roles: `cashier+`
```typescript
invoke('get_invoices', { token, filter?: {
  date_from?: string, date_to?: string, status?: string, search?: string
}}) → Invoice[]
```

### `get_invoice`
Roles: `cashier+`
```typescript
invoke('get_invoice', { token, id: number }) → Invoice
```

---

## Customers

### `get_customers`
Roles: All
```typescript
invoke('get_customers', { token, search?: string }) → Customer[]
```

### `create_customer`
Roles: `cashier+`
```typescript
invoke('create_customer', { token, request: {
  name: string, phone?: string, email?: string, address?: string, tin_number?: string
}}) → Customer
```

### `update_customer`
Roles: `manager+`
```typescript
invoke('update_customer', { token, id: number, request: Partial<CreateCustomerRequest> & { is_active?: boolean }}) → Customer
```

### `delete_customer`
Roles: `admin+`
```typescript
invoke('delete_customer', { token, id: number }) → void
```

---

## Suppliers

### `get_suppliers`
Roles: `manager+`
```typescript
invoke('get_suppliers', { token, search?: string }) → Supplier[]
```

### `create_supplier` / `update_supplier` / `delete_supplier`
Roles: `manager+` / `manager+` / `admin+`

---

## Reports

### `get_dashboard_stats`
Roles: All
```typescript
invoke('get_dashboard_stats', { token }) → DashboardStats
```

### `get_sales_report`
Roles: `manager+`
```typescript
invoke('get_sales_report', { token, filter: { date_from: string, date_to: string }}) → SalesReportSummary
```

### `get_inventory_report`
Roles: `manager+`
```typescript
invoke('get_inventory_report', { token }) → InventoryReportSummary
```

### `get_profit_loss_report`
Roles: `admin+`
```typescript
invoke('get_profit_loss_report', { token, filter: { date_from: string, date_to: string }}) → ProfitLossReport
```

### `get_vat_report`
Roles: `admin+`
```typescript
invoke('get_vat_report', { token, filter: { date_from: string, date_to: string }}) → VatReport
```

---

## Settings

### `get_settings`
Roles: All
```typescript
invoke('get_settings', { token }) → Record<string, string>
```

### `update_settings`
Roles: `super_admin`
```typescript
invoke('update_settings', { token, settings: Record<string, string> }) → void
```

---

## Data Types

```typescript
type Role = 'super_admin' | 'admin' | 'manager' | 'cashier' | 'viewer'

interface User {
  id: number; username: string; full_name: string; email?: string;
  role: Role; is_active: boolean; created_at: string; updated_at: string;
}

interface Product {
  id: number; category_id?: number; category_name?: string; sku: string;
  name: string; description?: string; unit: string; cost_price: number;
  selling_price: number; quantity: number; reorder_level: number;
  is_vat_exempt: boolean; is_active: boolean; created_at: string;
}

interface Sale {
  id: number; sale_number: string; customer_id?: number;
  customer_name?: string; user_id: number; cashier_name: string;
  sale_date: string; subtotal: number; discount_amount: number;
  vat_amount: number; total_amount: number; amount_paid: number;
  change_amount: number; payment_method: string; status: string;
  items: SaleItem[]; created_at: string;
}
```
