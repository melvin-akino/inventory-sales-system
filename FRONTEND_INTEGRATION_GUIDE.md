# Frontend Multi-Industry Integration Guide

## Overview

The frontend has been enhanced to support the multi-industry architecture. Users now see industry-specific products, categories, and custom fields based on their assigned industry.

## Components & Stores

### 1. **Updated Stores**

#### `useAuthStore` (src/stores/auth.js)
Now tracks user industry information:

```javascript
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()

// Access user's industry
console.log(auth.userIndustry) // { id: 1, code: 'electronics', name: 'Electronics & Lighting' }
console.log(auth.userIndustryId) // 1
console.log(auth.userIndustryCode) // 'electronics'
```

**Storage:**
- Industry data is automatically saved to `localStorage.userIndustry` on login
- Persists across page refreshes

#### `useIndustryStore` (src/stores/industry.js)
Manages industry data and configuration:

```javascript
import { useIndustryStore } from '@/stores/industry'

const industry = useIndustryStore()

// Get current industry info
industry.industryId        // Industry ID from user's session
industry.industryCode      // Industry code
industry.industryName      // "Electronics & Lighting"
industry.industryColor     // "#3B82F6"

// Load industry details from backend
await industry.loadIndustryDetails(industryId, token)

// Get custom attributes from backend
const attributes = industry.getIndustryAttributes()

// Get local configuration
const customFields = industry.getCustomFields() // From industryConfig
const categories = industry.getDefaultCategories()

// Check features
const hasPharmacy = industry.hasFeature('pharmacy')
```

### 2. **New Components**

#### `ProductForm.vue`
Complete product creation/editing form with industry-specific fields:

```vue
<template>
  <ProductForm 
    @success="handleProductSaved" 
    @cancel="goBack"
    :editingProduct="existingProduct"
  />
</template>

<script setup>
import ProductForm from '@/components/ProductForm.vue'

function handleProductSaved() {
  // Reload products list
  loadProducts()
}
</script>
```

**Features:**
- Basic product fields (SKU, name, description, category, unit)
- Pricing & stock (cost price, selling price, quantity, reorder level)
- VAT exempt checkbox
- **Dynamic industry-specific fields** based on user's industry
  - Electronics: wattage, color temperature, lumens, lifespan
  - Pharmacy: strength, dosage form, manufacturer, expiry date
  - Clothing: size, color, material, care instructions
  - etc.

#### `IndustrySwitcher.vue`
Allows super admins to switch between industries:

```vue
<template>
  <header>
    <IndustrySwitcher />
  </header>
</template>

<script setup>
import IndustrySwitcher from '@/components/IndustrySwitcher.vue'
</script>
```

**Features:**
- Dropdown showing all available industries
- Visual icons and colors for each industry
- Only visible to super_admin role
- Clicking industry reloads page in that industry context

#### `IndustryInfo.vue`
Displays detailed industry information:

```vue
<template>
  <IndustryInfo 
    :industryCode="'electronics'"
    :attributes="customAttributes"
  />
</template>

<script setup>
import IndustryInfo from '@/components/IndustryInfo.vue'
</script>
```

**Features:**
- Industry header with color and icon
- List of enabled features
- Custom attributes table
- Useful for admin dashboards

### 3. **Configuration**

#### `industryConfig.js` (src/utils/industryConfig.js)
Local industry metadata (doesn't require API calls):

```javascript
import { 
  getIndustryConfig, 
  getAllIndustryConfigs,
  hasIndustryFeature,
  getIndustryCustomFields,
  getIndustryDefaultCategories
} from '@/utils/industryConfig'

// Get config by ID or code
const config = getIndustryConfig('electronics')
const config = getIndustryConfig(1)

// Get all industries
const all = getAllIndustryConfigs()

// Check feature availability
const hasPharmacy = hasIndustryFeature('pharmacy', 'pharmacy')

// Get custom fields
const fields = getIndustryCustomFields('electronics')
// Returns: { 
//   wattage: { label: 'Wattage (W)', type: 'text', required: false },
//   lumens: { label: 'Brightness (Lumens)', type: 'number', required: false },
//   ...
// }

// Get default categories for industry
const cats = getIndustryDefaultCategories('pharmacy')
// Returns: ['Antibiotics', 'Painkillers', 'Cough & Cold', ...]
```

### 4. **API Functions**

#### `industriesApi` (src/utils/api.js)
New API client for industry endpoints:

```javascript
import { industriesApi } from '@/utils/api'

const token = useAuthStore().token

// Get all active industries
const industries = await industriesApi.getIndustries(token)

// Get industry details with attributes
const industry = await industriesApi.getIndustry(token, industryId)
// Returns: {
//   id: 1,
//   code: 'electronics',
//   name: 'Electronics & Lighting',
//   color: '#3B82F6',
//   attributes: [
//     { id: 1, attribute_name: 'wattage', attribute_label: 'Wattage (W)', attribute_type: 'text', ... },
//     ...
//   ]
// }

// Add custom attribute to industry (admin only)
await industriesApi.addIndustryAttribute(token, {
  industry_id: 1,
  attribute_name: 'warranty_months',
  attribute_label: 'Warranty (Months)',
  attribute_type: 'number',
  is_required: false,
  display_order: 5
})

// Assign user to industry (admin only)
await industriesApi.assignUserToIndustry(token, {
  user_id: 5,
  industry_id: 2
})
```

## Usage Examples

### Example 1: Show Product Form with Industry Fields

```vue
<template>
  <div class="p-6">
    <h1>Create Product</h1>
    <ProductForm @success="onSuccess" @cancel="onCancel" />
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import ProductForm from '@/components/ProductForm.vue'
import { useAppStore } from '@/stores/app'

const router = useRouter()
const appStore = useAppStore()

function onSuccess() {
  appStore.notify('Product created successfully!')
  router.push('/inventory/products')
}

function onCancel() {
  router.back()
}
</script>
```

### Example 2: Load and Display Industry Details

```vue
<template>
  <div>
    <IndustryInfo 
      :industryCode="industryStore.industryCode"
      :attributes="industryAttributes"
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'
import IndustryInfo from '@/components/IndustryInfo.vue'

const auth = useAuthStore()
const industryStore = useIndustryStore()
const industryAttributes = ref([])

onMounted(async () => {
  await industryStore.loadUserIndustry()
  if (industryStore.industryId) {
    const details = await industryStore.loadIndustryDetails(industryStore.industryId, auth.token)
    industryAttributes.value = details.attributes || []
  }
})
</script>
```

### Example 3: Conditional Rendering by Industry

```vue
<template>
  <div>
    <!-- Show pharmacy-specific UI only for pharmacy industry -->
    <div v-if="industryStore.hasFeature('pharmacy')">
      <PatientsList />
      <PrescriptionsWidget />
      <ControlledSubstancesLog />
    </div>

    <!-- Show general inventory -->
    <ProductsList />
  </div>
</template>

<script setup>
import { useIndustryStore } from '@/stores/industry'

const industryStore = useIndustryStore()
</script>
```

### Example 4: Create Product with Industry-Specific Fields

The `ProductForm` component automatically handles this. Custom fields are rendered based on `industryStore.getIndustryAttributes()`:

```javascript
// User in electronics industry sees: wattage, lumens, color temp, lifespan
// User in pharmacy industry sees: strength, dosage form, manufacturer, expiry date
// No code changes needed - completely dynamic!
```

## Data Flow

```
1. User Logs In
   ↓
2. Backend returns user with industry info
   ↓
3. Auth store saves user + industry to localStorage
   ↓
4. Industry store syncs with user's industry on app load
   ↓
5. ProductForm component loads attributes from industry store
   ↓
6. Form renders dynamic fields based on industry attributes
   ↓
7. User submits form
   ↓
8. Product saved with industry_id automatically set by backend
```

## Initialization

Add to your app's main initialization (e.g., `App.vue` `onMounted` or router guards):

```javascript
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'

async function initializeApp() {
  const auth = useAuthStore()
  const industry = useIndustryStore()

  // Restore session from localStorage
  const sessionRestored = await auth.restoreSession()

  if (sessionRestored && auth.userIndustry) {
    // Load user's industry info
    industry.loadUserIndustry()
    
    // Optionally load full industry details with attributes
    if (auth.token) {
      await industry.loadIndustryDetails(auth.userIndustryId, auth.token)
    }
  }
}
```

## Styling

All industry-specific elements use the industry's color from the config:

```javascript
// Automatically use industry color
const industryColor = computed(() => industry.industryColor)

// In template:
<button :style="{ backgroundColor: industryColor }">Submit</button>
```

Pre-defined colors:
- Electronics: `#3B82F6` (Blue)
- Pharmacy: `#EC4899` (Pink)
- Retail: `#8B5CF6` (Purple)
- Grocery: `#10B981` (Green)
- Clothing: `#F59E0B` (Amber)
- Furniture: `#6366F1` (Indigo)
- Automotive: `#EF4444` (Red)
- Cosmetics: `#EC4899` (Pink)

## Testing Checklist

- [ ] Login shows correct industry in user object
- [ ] Industry info persists in localStorage
- [ ] ProductForm loads for electronics (shows wattage/lumens fields)
- [ ] ProductForm loads for pharmacy (shows strength/dosage/expiry fields)
- [ ] IndustrySwitcher only visible to super_admin
- [ ] Switching industry reloads page with correct data
- [ ] Custom attributes from backend display in form
- [ ] Required field validation works
- [ ] Different field types render correctly (text, number, date, select, etc.)

## Common Issues & Fixes

### Issue: Industry attributes not loading
```javascript
// Make sure to await the load
await industry.loadIndustryDetails(industryId, token)
console.log(industry.customAttributes) // Should be populated
```

### Issue: Form not showing industry-specific fields
```javascript
// Ensure industry store is initialized
const industry = useIndustryStore()
onMounted(() => {
  industry.loadUserIndustry() // Must call this first
})
```

### Issue: LocalStorage not persisting
```javascript
// Check that localStorage is not being cleared
// Avoid calling localStorage.clear() in logout
// Instead, only remove specific keys:
localStorage.removeItem('auth_token')
localStorage.removeItem('userIndustry')
```

## Next Steps

1. **Update existing product list views** to show industry-specific columns
2. **Update category management** to show industry-specific categories
3. **Update reports** to filter by industry
4. **Add industry switcher** to admin dashboard
5. **Create industry admin panel** for managing attributes
6. **Add industry-specific validation rules** (e.g., require expiry date for pharmacy)
7. **Update navigation** to hide industry-specific menu items based on features
