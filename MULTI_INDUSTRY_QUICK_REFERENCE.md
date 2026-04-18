# Multi-Industry Quick Reference

## 🚀 Quick Start

### 1. Access User's Industry
```javascript
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
console.log(auth.userIndustry)      // Full industry object
console.log(auth.userIndustryId)    // ID (1-8)
console.log(auth.userIndustryCode)  // Code ('electronics', 'pharmacy', etc)
```

### 2. Load Industry Details
```javascript
import { useIndustryStore } from '@/stores/industry'

const industry = useIndustryStore()
await industry.loadIndustryDetails(industryId, token)
```

### 3. Check Industry Features
```javascript
const industry = useIndustryStore()

if (industry.hasFeature('pharmacy')) {
  // Show pharmacy UI
}

if (industry.hasFeature('prescriptions')) {
  // Show prescription features
}
```

### 4. Get Custom Fields
```javascript
const industry = useIndustryStore()

const fields = industry.getCustomFields()           // Local config
const attrs = industry.getIndustryAttributes()      // From backend
const cats = industry.getDefaultCategories()       // Default categories
```

---

## 📋 Industry Codes

| Code | Name | Color | Icon | Features |
|------|------|-------|------|----------|
| `electronics` | Electronics & Lighting | #3B82F6 | 💡 | inventory, sales, reports |
| `pharmacy` | Pharmacy & Healthcare | #EC4899 | 💊 | inventory, sales, **pharmacy**, **prescriptions** |
| `retail` | General Retail | #8B5CF6 | 🛍️ | inventory, sales, reports |
| `grocery` | Grocery & Food | #10B981 | 🛒 | inventory, sales, reports |
| `clothing` | Clothing & Fashion | #F59E0B | 👔 | inventory, sales, reports |
| `furniture` | Furniture & Home | #6366F1 | 🛋️ | inventory, sales, reports |
| `automotive` | Automotive & Parts | #EF4444 | 🚗 | inventory, sales, reports |
| `cosmetics` | Cosmetics & Beauty | #EC4899 | 💄 | inventory, sales, reports |

---

## 🏗️ Component Usage

### ProductForm (with industry fields)
```vue
<template>
  <ProductForm 
    @success="onProductCreated" 
    @cancel="goBack"
    :editingProduct="product"
  />
</template>

<script setup>
import ProductForm from '@/components/ProductForm.vue'

function onProductCreated() {
  // Product created with industry-specific fields automatically
}
</script>
```

### IndustrySwitcher (admin only)
```vue
<template>
  <header>
    <IndustrySwitcher /> <!-- Only shows for super_admin -->
  </header>
</template>

<script setup>
import IndustrySwitcher from '@/components/IndustrySwitcher.vue'
</script>
```

### IndustryInfo (display details)
```vue
<template>
  <IndustryInfo 
    :industryCode="'pharmacy'"
    :attributes="attributes"
  />
</template>

<script setup>
import IndustryInfo from '@/components/IndustryInfo.vue'
</script>
```

---

## 🔌 API Calls

### Get All Industries
```javascript
import { industriesApi } from '@/utils/api'

const industries = await industriesApi.getIndustries(token)
// Returns: [{ id, code, name, description, color, ... }, ...]
```

### Get Industry with Attributes
```javascript
const industry = await industriesApi.getIndustry(token, 1)
// Returns: { id, code, name, attributes: [...], ... }
```

### Add Custom Attribute
```javascript
await industriesApi.addIndustryAttribute(token, {
  industry_id: 1,
  attribute_name: 'warranty',
  attribute_label: 'Warranty (months)',
  attribute_type: 'number',
  is_required: false,
  display_order: 5
})
```

### Assign User to Industry
```javascript
await industriesApi.assignUserToIndustry(token, {
  user_id: 5,
  industry_id: 2  // Move user to pharmacy industry
})
```

---

## 🎨 Custom Fields by Industry

### Electronics 🔌
- `wattage` (text)
- `colorTemp` (select): Daylight, Warm White, Cool White, RGB
- `lumens` (number)
- `lifespanHours` (number)

### Pharmacy 💊
- `strength` (text) - Required
- `dosageForm` (select) - Required
- `manufacturer` (text) - Required
- `expiryDate` (date) - Required
- `genericName` (text)
- `ndc` (text)

### Clothing 👔
- `size` (select): XS, S, M, L, XL, XXL
- `color` (text)
- `material` (text)
- `careInstructions` (textarea)

### Grocery 🛒
- `unitSize` (text)
- `expiryDate` (date)
- `allergens` (textarea)

### Furniture 🛋️
- `material` (text)
- `dimensions` (text)
- `warranty` (number)
- `color` (text)

### Automotive 🚗
- `fitment` (text)
- `partNumber` (text) - Required
- `warranty` (number)

### Cosmetics 💄
- `skinType` (select): All, Dry, Oily, Combination, Sensitive
- `ingredients` (textarea)
- `volume` (number)
- `expiryDate` (date)

---

## 📊 Feature Flags by Industry

### Pharmacy-Only Features
```javascript
industry.hasFeature('pharmacy')              // ✓ pharmacy, ✗ others
industry.hasFeature('prescriptions')         // ✓ pharmacy, ✗ others
industry.hasFeature('controlledSubstances')  // ✓ pharmacy, ✗ others
```

### All Industries Have
```javascript
industry.hasFeature('inventory')   // All
industry.hasFeature('sales')       // All
industry.hasFeature('suppliers')   // All
industry.hasFeature('reports')     // All
```

### Pharmacy Unique
```javascript
industry.hasFeature('customers')   // All except pharmacy (uses patients)
```

---

## 🔒 Role-Based Access

### super_admin
- See all industries
- Switch between industries
- View all user data
- Create/manage industries
- Add custom attributes

### admin
- Locked to assigned industry
- Can't switch industries (in UI)
- Manage that industry's data
- Create/manage users in same industry

### manager
- Locked to assigned industry
- Manage products, sales, reports
- Can't access settings or users

### cashier
- Locked to assigned industry
- Create sales only
- View products

### viewer
- Locked to assigned industry
- Read-only access

---

## 🛠️ Common Tasks

### Show/Hide UI Based on Industry
```vue
<template>
  <!-- Pharmacy UI -->
  <div v-if="industry.hasFeature('pharmacy')">
    <PatientsList />
    <PrescriptionsWidget />
  </div>

  <!-- Electronics UI -->
  <div v-if="industry.industryCode === 'electronics'">
    <LightingSpecsWidget />
  </div>
</template>

<script setup>
import { useIndustryStore } from '@/stores/industry'
const industry = useIndustryStore()
</script>
```

### Style with Industry Color
```vue
<template>
  <button :style="{ backgroundColor: industry.industryColor }">
    Save
  </button>
</template>

<script setup>
import { useIndustryStore } from '@/stores/industry'
const industry = useIndustryStore()
</script>
```

### Filter Data by Industry
```javascript
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()

// All product queries automatically filter by:
// WHERE industry_id = auth.userIndustryId

// Get products (already filtered by backend)
const products = await productsApi.getProducts(token)
```

### Initialize on App Startup
```javascript
// In main.js or App.vue
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'

const auth = useAuthStore()
const industry = useIndustryStore()

// Restore login
await auth.restoreSession()

// Load industry if user exists
if (auth.isAuthenticated) {
  industry.loadUserIndustry()
  await industry.loadIndustryDetails(auth.userIndustryId, auth.token)
}
```

---

## 📱 LocalStorage Keys

```javascript
// Auth
localStorage.getItem('auth_token')      // Session token

// Industry
localStorage.getItem('userIndustry')    // User's assigned industry
localStorage.getItem('selectedIndustry') // Admin's selected industry
```

---

## 🚨 Common Mistakes

### ❌ Forgetting to await industry load
```javascript
// WRONG - attributes not ready
industry.loadIndustryDetails(id, token)
console.log(industry.customAttributes) // Empty!
```

```javascript
// RIGHT - wait for load
await industry.loadIndustryDetails(id, token)
console.log(industry.customAttributes) // Populated!
```

### ❌ Using auth.user.industry without checking
```javascript
// WRONG - might be undefined
const color = auth.user.industry.color
```

```javascript
// RIGHT - safe access
const color = auth.userIndustry?.color || '#3b82f6'
```

### ❌ Hardcoding field names
```javascript
// WRONG - not flexible
if (props.sku) { /* ... */ }
```

```javascript
// RIGHT - use store
const fields = industry.getCustomFields()
if (fields[fieldName]) { /* ... */ }
```

### ❌ Forgetting industry_id in filters
```javascript
// WRONG - gets data from other industries
const products = await productsApi.getProducts(token)
```

```javascript
// RIGHT - backend automatically filters, but be explicit in UI
const industry = useIndustryStore()
const products = await productsApi.getProducts(token, {
  filter: { industry_id: industry.industryId }
})
```

---

## ✅ Checklist for New Features

When adding a new feature, ensure:

- [ ] Feature check: `industry.hasFeature('featureName')`
- [ ] UI conditionally renders based on industry
- [ ] Custom fields properly scoped to industry
- [ ] Backend filters data by user's industry_id
- [ ] Proper role-based access control
- [ ] Industry color used for styling
- [ ] LocalStorage updated if needed
- [ ] API calls use correct industry context
- [ ] Documentation updated with feature flag

---

## 🔗 Documentation Links

- [Architecture](./MULTI_INDUSTRY_DESIGN.md) - Database, API, data isolation
- [Backend](./MULTI_INDUSTRY_IMPLEMENTATION.md) - Files changed, migrations
- [Frontend](./FRONTEND_INTEGRATION_GUIDE.md) - Components, stores, usage
- [Checklist](./MULTI_INDUSTRY_CHECKLIST.md) - Implementation steps, testing
- [This File](./MULTI_INDUSTRY_QUICK_REFERENCE.md) - Quick lookup
