# E-Commerce Testing & Deployment Guide

## ✅ Implementation Complete

All 3 remaining tasks are complete:

### 1. Vue Frontend Components (7 components created)
- ✅ **Shop.vue** - Product catalog with search/filter/categories
- ✅ **ProductDetail.vue** - Individual product page with quantity selector
- ✅ **Cart.vue** - Shopping cart with shipping address form
- ✅ **CheckoutPayment.vue** - Mock payment gateway with 4 payment methods
- ✅ **OrderConfirmation.vue** - Order success page with tracking info
- ✅ **Auth.vue** - Register/Login page for e-commerce customers
- ✅ **AdminOrders.vue** - Admin dashboard for order management & status updates

### 2. API Utilities (src/utils/ecommerce.js)
- ✅ ecommerceAuth (register, login, getCustomer)
- ✅ ecommerceProducts (getAll, search, filterByCategory)
- ✅ ecommerceCheckout (createOrder, mockPayment)
- ✅ adminOrders (getAll, updateStatus)

### 3. Pinia Store (src/stores/ecommerce.js)
- ✅ State management (customer, token, cart, orders)
- ✅ Computed properties (cartTotal, cartItemCount)
- ✅ Auth methods (register, login, logout)
- ✅ Cart methods (addToCart, removeFromCart, updateQuantity, clearCart)
- ✅ Checkout flow (checkout, processPayment)

### 4. Router Configuration (src/router/index.js)
- ✅ E-commerce public routes (/shop, /ecommerce/login, /product/:id)
- ✅ Protected customer routes (/cart, /checkout/payment, /order-confirmation)
- ✅ Admin routes (/admin/ecommerce)
- ✅ Route guards for e-commerce auth

---

## 🚀 Deployment Instructions

### Prerequisites
- Docker & Docker Compose installed
- Node.js 16+ (for local dev)
- Rust 1.70+ (for server)

### Step 1: Install Dependencies

```bash
# Frontend
npm install

# Server (already in Cargo.toml)
cd server
cargo build
cd ..
```

### Step 2: Build Docker Image

```bash
# Build backend image
docker build -t lumisync-server -f server/Dockerfile .

# Or use docker-compose
docker-compose build
```

### Step 3: Run with Docker Compose

```bash
docker-compose up -d
```

This starts:
- Backend API on `http://localhost:3000`
- Frontend on `http://localhost:5173`

### Step 4: Verify Services

```bash
# Check backend health
curl http://localhost:3000/health
# Response: OK

# Frontend is accessible at
http://localhost:5173
```

---

## 🧪 Testing Checklist

### Customer Registration & Login
- [ ] Visit `http://localhost:5173/#/ecommerce/login`
- [ ] Click "Register" tab
- [ ] Fill form with test data:
  - Email: `test@example.com`
  - Password: `Test123456`
  - First Name: John
  - Last Name Doe
  - Phone: 09123456789
- [ ] Click "Create Account"
- [ ] Verify redirected to /shop
- [ ] Verify logged in (name shown in header)

### Product Browsing
- [ ] Browse products on /shop
- [ ] Search for "LED" (should filter products)
- [ ] Filter by category "LED Bulbs"
- [ ] Click "View" on a product → ProductDetail page
- [ ] Increment/decrement quantity
- [ ] Click "Add to Cart" → verify success toast

### Shopping Cart
- [ ] Navigate to /cart
- [ ] Verify cart items display
- [ ] Update quantities, verify total updates
- [ ] Remove item → verify removed
- [ ] Try "Clear Cart" → confirm empty

### Checkout & Mock Payment
- [ ] Add 2-3 products to cart
- [ ] Go to /cart
- [ ] Fill shipping address form:
  - Address: 123 Main Street
  - City: Manila
  - State: NCR
  - ZIP: 1000
- [ ] Select payment method (Card, GCash, Maya, or Bank Transfer)
- [ ] Click "Proceed to Payment"
- [ ] Verify order created with order_number
- [ ] On payment page, select payment method
- [ ] Click "Process Payment"
- [ ] Verify 95% success rate (should mostly succeed):
  - ✓ Success: Green banner + transaction ID
  - ✗ Failure: Red banner (rare, ~5%)
- [ ] If success, redirected to /order-confirmation
- [ ] Verify order details display

### Inventory Deduction
**Critical Test**: Verify stock was deducted

```bash
# 1. Check product stock BEFORE checkout
curl -X POST http://localhost:3000/api/ecommerce/products \
  -H "Content-Type: application/json" \
  -d '{"filter": {"search": "LED"}}'

# Note the stock for a product (e.g., "stock": 45)

# 2. Complete checkout with that product (qty 3)

# 3. Check stock AFTER checkout
curl -X POST http://localhost:3000/api/ecommerce/products \
  -H "Content-Type: application/json" \
  -d '{"filter": {"search": "LED"}}'

# Stock should now be 45 - 3 = 42
```

### Admin Order Management
- [ ] Login to admin panel (/login) with admin credentials
- [ ] Navigate to /admin/ecommerce
- [ ] Verify all customer orders display
- [ ] Click "View" on an order → modal shows items
- [ ] Change order status (pending → processing → shipped → delivered)
- [ ] Verify status updates in real-time

### Payment Method Verification
Test all 4 payment methods:
- [ ] Card payment
- [ ] GCash payment
- [ ] Maya payment
- [ ] Bank Transfer payment

---

## 📊 API Testing (via cURL or Postman)

### 1. Customer Registration
```bash
curl -X POST http://localhost:3000/api/ecommerce/register \
  -H "Content-Type: application/json" \
  -d '{
    "request": {
      "email": "customer@test.com",
      "password": "SecurePass123",
      "first_name": "John",
      "last_name": "Doe",
      "phone": "09123456789"
    }
  }'

# Response: Customer object + id
```

### 2. Customer Login
```bash
curl -X POST http://localhost:3000/api/ecommerce/login \
  -H "Content-Type: application/json" \
  -d '{
    "request": {
      "email": "customer@test.com",
      "password": "SecurePass123"
    }
  }'

# Response: { "token": "uuid-string", "customer": {...} }
# Save token for next requests
```

### 3. Get Products
```bash
curl -X POST http://localhost:3000/api/ecommerce/products \
  -H "Content-Type: application/json" \
  -d '{
    "filter": {
      "search": "LED",
      "category_id": 1
    }
  }'

# Response: Array of products with id, name, price, stock
```

### 4. Create Order (Checkout)
```bash
curl -X POST http://localhost:3000/api/ecommerce/checkout \
  -H "Content-Type: application/json" \
  -d '{
    "token": "CUSTOMER_TOKEN_HERE",
    "request": {
      "items": [
        { "product_id": 1, "quantity": 2 },
        { "product_id": 5, "quantity": 1 }
      ],
      "shipping_address": "123 Main St, Manila, NCR 1000",
      "payment_method": "card",
      "discount_amount": 0
    }
  }'

# Response: Order object with order_number, total_amount, items
# Save order_id for payment processing
```

### 5. Process Mock Payment
```bash
curl -X POST http://localhost:3000/api/ecommerce/mock-payment \
  -H "Content-Type: application/json" \
  -d '{
    "token": "CUSTOMER_TOKEN_HERE",
    "request": {
      "order_id": 1,
      "payment_method": "card"
    }
  }'

# Response: { "success": true/false, "payment_status": "completed/failed", "transaction_id": "..." }
```

### 6. Admin: Get All Orders
```bash
curl -X POST http://localhost:3000/api/admin/orders \
  -H "Content-Type: application/json" \
  -d '{
    "token": "ADMIN_TOKEN_HERE"
  }'

# Response: Array of all orders from all customers
```

### 7. Admin: Update Order Status
```bash
curl -X POST http://localhost:3000/api/admin/orders/update-status \
  -H "Content-Type: application/json" \
  -d '{
    "token": "ADMIN_TOKEN_HERE",
    "request": {
      "order_id": 1,
      "status": "shipped"
    }
  }'

# Response: Updated order object
```

---

## 🔍 Debugging

### Backend Logs
```bash
docker-compose logs -f server
# Check for errors, connection issues, SQL errors
```

### Frontend Console
```
Open browser DevTools (F12) → Console tab
Check for JavaScript errors, API request failures
```

### Database Inspection
```bash
# Access SQLite database
sqlite3 /data/lumisync.db

# Check e-commerce tables
SELECT * FROM ecommerce_customers;
SELECT * FROM orders;
SELECT * FROM order_items;
SELECT * FROM payments;

# Verify inventory deduction
SELECT id, name, quantity FROM products WHERE id = 1;
```

### Common Issues

**Issue**: "Product not found" on checkout
- **Fix**: Verify product exists and is_active = 1

**Issue**: "Insufficient stock" error
- **Fix**: Check product.quantity > order quantity

**Issue**: Login fails for new customer
- **Fix**: Verify email not already registered, password >= 6 chars

**Issue**: Payment always fails
- **Fix**: Mock gateway has 95% success rate, try again (5% intentional failures for testing)

---

## 📈 Performance Tuning

### Database Indexes (Already Applied)
```sql
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_order_number ON orders(order_number);
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_shopping_carts_customer_id ON shopping_carts(customer_id);
```

### Frontend Optimization
- Components use lazy loading (async import in router)
- Cart state cached in Pinia (no extra API calls)
- Product search throttled (100ms debounce)

### Backend Optimization
- Connection pooling (SQLite WAL mode enabled)
- Prepared statements prevent SQL injection
- Response caching on public endpoints possible

---

## 🔐 Security Checklist

- ✅ Passwords hashed with bcrypt (cost 12)
- ✅ Sessions use UUID tokens (cryptographically random)
- ✅ 30-day session expiry
- ✅ CORS enabled (modify for production)
- ✅ Foreign key constraints enforced
- ✅ Input validation on all endpoints
- ✅ Inventory locked during checkout (no race conditions)

**For Production:**
- [ ] Use HTTPS/TLS
- [ ] Set restrictive CORS (whitelist domain)
- [ ] Use environment variables for secrets
- [ ] Enable audit logging
- [ ] Rate limit API endpoints
- [ ] Add payment provider credentials to .env

---

## 📝 Next Steps

1. **Real Payment Gateway Integration**
   - Replace `mock_payment()` with PayMongo API calls
   - Add environment variables for API keys
   - Test with actual payment provider sandbox

2. **Email Notifications**
   - Send order confirmation emails
   - Send shipping tracking emails
   - Use SendGrid or AWS SES

3. **Frontend Enhancements**
   - Add product images/gallery
   - Product reviews & ratings
   - Wishlist feature
   - Order history page
   - Customer profile management

4. **Admin Features**
   - Inventory alerts (low stock warnings)
   - Sales analytics dashboard
   - Customer management
   - Refund processing
   - Bulk order operations

5. **Monitoring & Analytics**
   - Track conversion rates
   - Monitor API performance
   - Log payment transactions
   - Customer behavior analytics

---

## 📞 Support

For issues during testing:
1. Check `/data/lumisync.db` for data integrity
2. Review backend logs in `docker-compose logs server`
3. Check frontend console for JavaScript errors
4. Verify all containers running: `docker-compose ps`
5. Restart services: `docker-compose down && docker-compose up -d`
