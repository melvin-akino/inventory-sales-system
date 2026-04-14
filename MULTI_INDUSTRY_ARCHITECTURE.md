# 🏢 Multi-Industry POS & Inventory Platform

## Overview

This is a next-generation Point of Sale (POS) and inventory management system designed to serve multiple industries through a single, flexible codebase. The platform supports industry-specific features, custom data models, and tailored user interfaces while maximizing code reuse.

## Supported Industries

### 1. **Electrical Supply Store** (Original)
- 🔌 Electrical equipment and lighting
- Features: POS, Inventory, Sales, Customers, Reports
- Categories: LED Bulbs, Fluorescent, Downlights, Accessories

### 2. **Pharmacy** (New)
- 💊 Medications and health products
- Features: All core + Prescriptions, Patients, Expiry Tracking, Controlled Substances
- Categories: Antibiotics, Painkillers, Vitamins, Topical, etc.
- Special: Prescription management, patient profiles, expiry dates, controlled substance logging

### 3. **Retail** (Planned)
- 🛍️ General retail and fashion
- Features: POS, Inventory, Customers, Promotions, Loyalty
- Categories: Clothing, Footwear, Electronics, Accessories

### 4. **Restaurant/Cafe** (Planned)
- 🍽️ Food and beverage service
- Features: POS, Orders, Menu, Kitchen Display, Reservations
- Categories: Appetizers, Main Course, Desserts, Beverages

### 5. **Supermarket** (Planned)
- 🏪 Large-scale retail with groceries
- Features: All core + Bulk operations, Promotions, Loyalty
- Categories: Groceries, Produce, Meat, Dairy, Snacks

## Architecture

### Directory Structure

```
src/
├── views/
│   ├── Login.vue                 # Universal login (shows company branding)
│   ├── IndustrySelection.vue     # Industry chooser
│   ├── Dashboard.vue             # Industry-aware dashboard
│   ├── Settings.vue              # Global and industry-specific settings
│   ├── sales/
│   │   └── POS.vue              # Generic POS (reusable)
│   ├── pharmacy/
│   │   ├── PharmacyPOS.vue       # Pharmacy-specific POS
│   │   ├── Patients.vue          # Pharmacy patient management
│   │   ├── Prescriptions.vue     # Prescription management
│   │   └── ExpiredItems.vue      # Expiry tracking
│   ├── inventory/
│   │   ├── Products.vue          # Generic products (reusable)
│   │   └── Categories.vue        # Generic categories
│   ├── reports/
│   │   └── SalesReport.vue       # Reusable reports
│   └── ... (other generic views)
│
├── utils/
│   ├── industryConfig.js         # Industry definitions and configs
│   ├── pharmacyModels.js         # Pharmacy-specific data models
│   ├── api.js                    # Universal API client
│   ├── format.js                 # Shared utilities
│   └── ...
│
├── stores/
│   ├── auth.js                   # Authentication store
│   ├── industry.js               # Industry context store (NEW)
│   └── ...
│
├── components/
│   ├── layout/
│   │   ├── AppLayout.vue         # Main layout (renders based on industry)
│   │   └── Sidebar.vue           # Industry-aware menu
│   └── ...
│
├── router/
│   └── index.js                  # Routes with industry awareness
│
└── ...
```

### Core Concepts

#### 1. **Industry Configuration**
Each industry is defined in `industryConfig.js`:
```javascript
pharmacy: {
  id: 'pharmacy',
  name: 'Pharmacy',
  icon: '💊',
  color: '#10b981',
  features: {
    pos: true,
    prescriptions: true,    // Pharmacy-specific
    patients: true,         // Pharmacy-specific
    expiry_tracking: true,  // Pharmacy-specific
  },
  customFields: { ... },
  defaultCategories: [ ... ],
}
```

#### 2. **Shared Core Features**
Features reused across all industries:
- Point of Sale (POS)
- Inventory Management
- Customer Management
- Sales History
- Reports & Analytics
- User Management
- Settings

#### 3. **Industry-Specific Features**
Features unique to specific industries:
- **Pharmacy:** Prescriptions, Patients, Expiry Dates, Controlled Substances
- **Restaurant:** Menu, Orders, Kitchen Display, Reservations
- **Retail:** Promotions, Loyalty Programs, Size/Color Variants
- **Supermarket:** Bulk Operations, Expiry Tracking, Barcodes

#### 4. **Dynamic UI Routing**
Router loads different components based on industry:

```javascript
// In router, check current industry
const industryId = useIndustryStore().currentIndustry

// Load industry-specific POS
if (industryId === 'pharmacy') {
  component: () => import('@/views/pharmacy/PharmacyPOS.vue')
} else {
  component: () => import('@/views/sales/POS.vue')
}
```

#### 5. **Industry Store (Pinia)**
Centralized industry context:
```javascript
import { useIndustryStore } from '@/stores/industry'

const industry = useIndustryStore()
industry.setIndustry('pharmacy')
industry.hasFeature('prescriptions') // true for pharmacy
industry.getCustomFields() // Get industry-specific fields
```

## Implementation Guide

### Adding a New Industry

1. **Define Industry Config** (`src/utils/industryConfig.js`):
```javascript
restaurant: {
  id: 'restaurant',
  name: 'Restaurant',
  icon: '🍽️',
  features: { ... },
  customFields: { ... },
  defaultCategories: [ ... ],
}
```

2. **Create Industry Models** (e.g., `src/utils/restaurantModels.js`):
```javascript
export const MENU_ITEM_SCHEMA = { ... }
export const TABLE_SCHEMA = { ... }
export const ORDER_SCHEMA = { ... }
```

3. **Create Industry-Specific Views** (`src/views/restaurant/`):
```
src/views/restaurant/
├── RestaurantPOS.vue
├── MenuManagement.vue
├── Orders.vue
├── Tables.vue
└── KitchenDisplay.vue
```

4. **Update Router** to include new routes:
```javascript
{
  path: 'restaurant/menu',
  component: () => import('@/views/restaurant/MenuManagement.vue'),
}
```

5. **Create Data Models in Backend** (Rust migrations)

6. **Test with Sample Data**

### Extending Pharmacy System

The pharmacy implementation includes:

#### Patient Management
- Patient profiles with allergies and medical conditions
- Insurance information
- Contact details
- Purchase history

#### Prescription Management
- Load prescription by Rx number
- Link medications to prescriptions
- Track refills
- Expiry management

#### Medication-Specific POS
- Prescription requirements display
- Expiry date alerts
- Controlled substance logging
- Patient allergy warnings
- Insurance coverage info

#### Regulatory Compliance
- Controlled substance logging with witness
- Audit trail for all transactions
- DEA compliance features
- Insurance claim tracking

## API Endpoints

### Industry-Agnostic (All Industries)
```
POST /api/login
GET /api/get-products
POST /api/create-sale
GET /api/get-sales
GET /api/get-customers
... (core endpoints)
```

### Pharmacy-Specific
```
GET /api/pharmacy/patients
POST /api/pharmacy/create-patient
GET /api/pharmacy/prescriptions
POST /api/pharmacy/create-prescription
GET /api/pharmacy/refills
POST /api/pharmacy/complete-refill
POST /api/pharmacy/log-controlled-substance
GET /api/pharmacy/expiry-alerts
```

### Restaurant-Specific (When Implemented)
```
GET /api/restaurant/menu
POST /api/restaurant/create-menu-item
GET /api/restaurant/orders
POST /api/restaurant/create-order
POST /api/restaurant/send-to-kitchen
GET /api/restaurant/tables
POST /api/restaurant/reserve-table
```

## Database Schema

### Multi-Tenancy (Future)
Each customer/business has:
- Separate database or database schema
- Isolated user access
- Independent configuration

### Industry-Specific Tables
Additional tables for industry-specific features:

**Pharmacy:**
- patients
- prescriptions
- prescription_items
- controlled_substance_log
- pharmacy_sales (extends base sales)

**Restaurant:**
- menu_items
- orders
- order_items
- tables
- reservations
- kitchen_items

## Development Workflow

### Step 1: Core Development (✅ Completed)
- [x] Base system (Electrical)
- [x] POS, Inventory, Sales, Reports
- [x] User authentication
- [x] Industry configuration system

### Step 2: First Reusable Industry (In Progress)
- [x] Pharmacy system setup
- [x] Pharmacy POS with prescriptions
- [x] Patient management
- [ ] Database migrations for pharmacy
- [ ] Backend API for pharmacy
- [ ] Prescription management UI
- [ ] Expiry tracking

### Step 3: Second Industry Replication (Planned)
- [ ] Retail system
- [ ] Restaurant system
- [ ] Template system for new industries

### Step 4: Advanced Features (Planned)
- [ ] Multi-tenancy
- [ ] Advanced permissions
- [ ] Custom field system
- [ ] Plugin architecture
- [ ] API marketplace

## Component Reuse Matrix

| Component | Electrical | Pharmacy | Retail | Restaurant |
|-----------|-----------|----------|--------|------------|
| Login | ✓ | ✓ | ✓ | ✓ |
| POS | ✓ | ✓* | ✓ | ✗ |
| Inventory | ✓ | ✓* | ✓ | ✓ |
| Customers | ✓ | ✓* | ✓ | ✓ |
| Sales | ✓ | ✓* | ✓ | ✓ |
| Reports | ✓ | ✓ | ✓ | ✓ |

*= Industry-specific variant exists

## Configuration by Industry

### Electrical
```javascript
customFields: {
  voltage: 'Voltage (V)',
  wattage: 'Wattage (W)',
  warranty: 'Warranty (months)',
}
```

### Pharmacy
```javascript
customFields: {
  batch_number: 'Batch Number',
  expiry_date: 'Expiry Date',
  generic_name: 'Generic Name',
  strength: 'Strength/Dosage',
}
```

### Restaurant
```javascript
customFields: {
  recipe: 'Recipe/Ingredients',
  preparation_time: 'Prep Time (min)',
  allergens: 'Allergens',
}
```

## Future Enhancements

1. **Plugin System**: Allow third-party developers to add industries
2. **Custom Fields UI**: Dynamic custom field builder
3. **Workflow Automation**: Industry-specific workflows
4. **Integration Hub**: Connect to external services (accountants, suppliers)
5. **Mobile App**: React Native app using same architecture
6. **Cloud Deployment**: SaaS platform with multi-tenancy
7. **AI Features**: Inventory optimization, sales forecasting per industry

## Technology Stack

- **Frontend:** Vue 3, Vite, Tailwind CSS, Pinia
- **Backend:** Rust, Axum, SQLite
- **Architecture:** Component-based, modular, extensible

## Next Steps

1. Complete pharmacy database migrations
2. Implement pharmacy backend APIs
3. Test pharmacy POS end-to-end
4. Document pharmacy API
5. Start retail system
6. Create template for new industries
7. Build industry onboarding flow

## Contributing

When adding new industries:
1. Create industry config in `industryConfig.js`
2. Create models file (e.g., `restaurantModels.js`)
3. Create views folder (`src/views/restaurant/`)
4. Create backend migrations and APIs
5. Update router with new routes
6. Test and document
7. Submit PR with all above

---

**Created:** April 2026
**Version:** 2.0.0 (Multi-Industry Platform)
**Status:** In Active Development
