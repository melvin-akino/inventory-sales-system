# 🔍 Cart Items Not Showing - Diagnostic Guide

## What Should Happen

When you navigate to **Point of Sale (POS)**:

1. **Left Panel** - Should show products in a grid
   - Product name
   - SKU and category
   - Price
   - Stock available
   - Click to add to cart

2. **Right Panel** - Shopping Cart
   - Should show items you added
   - Quantity controls (+/-)
   - Remove button
   - Totals at bottom

## Troubleshooting Steps

### Step 1: Check if Products are Loading

1. Open **F12** (Developer Tools)
2. Go to **Network** tab
3. Refresh page
4. Look for requests to `/api/get-products`
5. Check the response:
   - ✅ **200 status** = Products loaded successfully
   - ❌ **Error status** = API call failed
   - ❌ **No request** = API wasn't called

### Step 2: Check Browser Console for Errors

1. Open **F12** (Developer Tools)
2. Go to **Console** tab
3. Look for red error messages
4. Common errors:
   - `Failed to fetch` = Backend unreachable
   - `undefined` = Data structure issue
   - `Cannot read property` = Missing data

### Step 3: Check Backend Logs

```bash
docker logs lumisync-backend
```

Should show:
- ✅ `Opening database at /data/lumisync.db`
- ✅ `LumiSync API server listening on 0.0.0.0:3000`
- ✅ No error messages

### Step 4: Verify Database has Products

```bash
docker exec lumisync-backend sh -c "sqlite3 /data/lumisync.db 'SELECT COUNT(*) FROM products;'"
```

Should return a number > 0

If it returns 0, products weren't created in the initial setup.

## Common Issues & Solutions

### Issue: Products Grid is Empty

**Cause:** No products in database

**Solution:**
1. Go to **Inventory → Products**
2. Click **Add Product**
3. Fill in details:
   - Name
   - SKU
   - Category
   - Selling Price
   - Quantity
4. Save
5. Go back to POS
6. Products should now appear

### Issue: Products Load but Cart Items Don't Show

**Cause:** Vue component not rendering correctly

**Solution:**
1. Hard refresh: `Ctrl + Shift + R`
2. Clear cache: F12 → Application → Clear Site Data
3. Close and reopen browser tab

### Issue: Cart shows "Cart is empty" even after clicking products

**Cause:** Click handler not working or API error

**Solution:**
1. Check browser console (F12)
2. Look for errors when you click a product
3. Check Network tab to see API calls
4. Try refreshing and clicking again

### Issue: API calls return 404 or 500

**Cause:** Backend API endpoint not working

**Solution:**
1. Restart backend:
   ```bash
   docker-compose restart lumisync-backend
   ```
2. Check backend logs:
   ```bash
   docker logs lumisync-backend
   ```
3. Ensure database exists:
   ```bash
   docker exec lumisync-backend ls -la /data/
   ```

## Testing the Flow

### Manual Test

1. **Open POS page**
   - URL: http://localhost:8080/#/sales/pos

2. **Look at left panel**
   - Should see product grid with items
   - If not, see "Products Grid is Empty" above

3. **Click a product**
   - Should see it appear in cart on right side
   - Quantity should show as 1
   - Amount Paid should update

4. **Adjust quantity**
   - Click + button to increase
   - Click - button to decrease
   - Input field should update

5. **See totals update**
   - Subtotal should change
   - VAT should update
   - Total should reflect changes

## API Endpoint Verification

To verify the API is working, test each endpoint:

```bash
# Test 1: Get Products
curl http://localhost:8080/api/get-products \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"token":"YOUR_TOKEN_HERE"}'

# Test 2: Get Categories
curl http://localhost:8080/api/get-categories \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"token":"YOUR_TOKEN_HERE"}'
```

## Step-by-Step Diagnosis

### If Nothing Shows:

1. ✅ Containers running?
   ```bash
   docker ps
   ```

2. ✅ Backend healthy?
   ```bash
   docker logs lumisync-backend | tail -5
   ```

3. ✅ Frontend serving?
   ```bash
   docker logs lumisync-frontend | tail -5
   ```

4. ✅ Can access app?
   - Open http://localhost:8080
   - See login page?

5. ✅ Logged in?
   - Username: admin
   - Password: Admin@123

6. ✅ In POS page?
   - Click Sales → Point of Sale

### If Products Show But Cart Doesn't:

1. Open F12 Console
2. Try clicking a product
3. Check for JavaScript errors
4. Check Network tab for API calls

## What the Cart Should Look Like

```
┌─────────────────────────────────┐
│    Shopping Cart              X │
│    1 item(s) in cart            │
├─────────────────────────────────┤
│ Product Name                    │
│ ₱78.00 each                     │
│                                 │
│ Quantity: [−] 1 [+]             │
│           of 90 available       │
│                                 │
│ Discount: [0] %                 │
│                                 │
│ Subtotal: ₱78.00                │
├─────────────────────────────────┤
│ Subtotal        ₱78.00          │
│ Discount                 ₱0     │
│ VAT (12%)       ₱9.36           │
│ TOTAL        ₱87.36             │
├─────────────────────────────────┤
│ Payment Method: Cash            │
│ Amount Paid: [87]               │
│ Change: ₱0.00                   │
│                                 │
│ [✓ Complete Sale]               │
└─────────────────────────────────┘
```

## Final Check

Run this complete flow:

1. Hard refresh: `Ctrl + Shift + R`
2. Wait 5 seconds
3. Go to POS
4. Click a product
5. See it in cart
6. Click it 3 more times
7. See quantity = 4
8. See subtotal = 4 × price
9. See totals updated

**If all these work, system is functioning correctly!**

---

**What specific issue are you seeing?**
- Products not loading on left?
- Cart not showing items?
- Items added but not visible?
- Something else?

Let me know and I can provide more specific help!
