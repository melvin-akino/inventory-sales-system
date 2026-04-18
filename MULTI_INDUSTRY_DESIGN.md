# Multi-Industry Product Support Architecture

## Overview

The system has been redesigned to support multiple industries with industry-specific products, categories, and custom attributes. This enables a single platform to serve electronics retailers, pharmacies, clothing stores, grocery chains, and more.

## Database Schema Changes (Migration V6)

### New Tables

#### 1. **industries**
Defines available industries in the system.

```sql
CREATE TABLE industries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT UNIQUE NOT NULL,           -- Unique identifier (e.g., 'electronics', 'pharmacy')
    name TEXT UNIQUE NOT NULL,           -- Display name
    description TEXT,                    -- Industry description
    icon TEXT,                          -- Optional icon reference
    color TEXT,                         -- Brand color (e.g., '#3B82F6')
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Pre-seeded Industries:**
- `electronics` - Electronics & Lighting
- `pharmacy` - Pharmacy & Healthcare
- `retail` - General Retail
- `grocery` - Grocery & Food
- `clothing` - Clothing & Fashion
- `furniture` - Furniture & Home
- `automotive` - Automotive & Parts
- `cosmetics` - Cosmetics & Beauty

#### 2. **product_attributes**
Defines custom attributes for each industry's products.

```sql
CREATE TABLE product_attributes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    industry_id INTEGER NOT NULL,
    attribute_name TEXT NOT NULL,       -- Internal name (e.g., 'wattage')
    attribute_label TEXT NOT NULL,      -- User-facing label (e.g., 'Wattage (W)')
    attribute_type TEXT NOT NULL,       -- text, number, checkbox, select, date, textarea
    is_required INTEGER NOT NULL DEFAULT 0,
    display_order INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (industry_id) REFERENCES industries(id) ON DELETE CASCADE,
    UNIQUE(industry_id, attribute_name)
);
```

**Example Attributes by Industry:**

| Industry | Attributes |
|----------|-----------|
| Electronics | wattage, color_temp, lumens, lifespan_hours |
| Pharmacy | strength, dosage_form, manufacturer, expiry_date |
| Clothing | size, color, material, care_instructions |
| Grocery | unit_size, nutritional_info, allergens, best_by_date |

#### 3. **product_attribute_values**
Stores custom attribute values for individual products.

```sql
CREATE TABLE product_attribute_values (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    attribute_value TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (attribute_id) REFERENCES product_attributes(id) ON DELETE CASCADE,
    UNIQUE(product_id, attribute_id)
);
```

### Modified Tables

#### users
```sql
ALTER TABLE users ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;
```
- Users are now assigned to a specific industry
- All products, categories, and settings are filtered by the user's industry

#### products
```sql
ALTER TABLE products ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;
```
- Products belong to a specific industry
- Queries automatically filter by user's industry_id

#### categories
```sql
ALTER TABLE categories ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;
```
- Categories can be industry-specific
- E.g., "Antibiotics" category is specific to pharmacy, "LED Bulbs" to electronics

#### settings
```sql
ALTER TABLE settings ADD COLUMN industry_id INTEGER REFERENCES industries(id) DEFAULT NULL;
```
- Company settings can vary per industry

## API Endpoints

### Industry Management

#### `GET /api/get-industries`
Retrieve all active industries.

**Response:**
```json
[
  {
    "id": 1,
    "code": "electronics",
    "name": "Electronics & Lighting",
    "description": "Retail of electronic devices and lighting fixtures",
    "color": "#3B82F6",
    "is_active": true,
    "created_at": "2024-04-15T08:00:00"
  },
  {
    "id": 2,
    "code": "pharmacy",
    "name": "Pharmacy & Healthcare",
    "description": "Pharmacy and healthcare products distribution",
    "color": "#EC4899",
    "is_active": true,
    "created_at": "2024-04-15T08:00:00"
  }
]
```

#### `POST /api/get-industry`
Get detailed industry info with attributes.

**Request:**
```json
{
  "request": {},
  "id": 1
}
```

**Response:**
```json
{
  "id": 1,
  "code": "electronics",
  "name": "Electronics & Lighting",
  "description": "Retail of electronic devices and lighting fixtures",
  "color": "#3B82F6",
  "is_active": true,
  "created_at": "2024-04-15T08:00:00",
  "attributes": [
    {
      "id": 1,
      "attribute_name": "wattage",
      "attribute_label": "Wattage (W)",
      "attribute_type": "text",
      "is_required": false,
      "display_order": 1
    },
    {
      "id": 2,
      "attribute_name": "lumens",
      "attribute_label": "Brightness (Lumens)",
      "attribute_type": "number",
      "is_required": false,
      "display_order": 3
    }
  ]
}
```

#### `POST /api/add-industry-attribute`
Add a custom attribute to an industry (Admin only).

**Request:**
```json
{
  "request": {
    "industry_id": 1,
    "attribute_name": "warranty_months",
    "attribute_label": "Warranty (Months)",
    "attribute_type": "number",
    "is_required": false,
    "display_order": 5
  },
  "token": "..."
}
```

#### `POST /api/assign-user-to-industry`
Assign a user to an industry (Admin only).

**Request:**
```json
{
  "request": {
    "user_id": 5,
    "industry_id": 1
  },
  "token": "..."
}
```

### Authentication (Updated)

#### `POST /api/login`
Login now returns industry information in the user object.

**Response:**
```json
{
  "token": "uuid-string",
  "user": {
    "id": 1,
    "username": "john",
    "full_name": "John Doe",
    "email": "john@example.com",
    "role": "manager",
    "is_active": true,
    "industry": {
      "id": 1,
      "code": "electronics",
      "name": "Electronics & Lighting"
    },
    "created_at": "2024-04-10T12:00:00",
    "updated_at": "2024-04-15T08:30:00"
  }
}
```

#### `POST /api/get-current-user`
Also returns industry information.

### Product Management (Updated)

#### Products are now filtered by user's industry

**Example: Get Products**
```json
{
  "request": {},
  "filter": {
    "search": "LED",
    "active_only": true
  },
  "token": "..."
}
```

The response will only include products from the logged-in user's industry.

## Data Isolation Pattern

### How It Works

1. **User Authentication**
   ```rust
   // In auth.rs
   let (id, ..., industry_id, industry_code, industry_name, ...) = 
       db.query_row("SELECT ..., industry_id, ... FROM users LEFT JOIN industries");
   ```

2. **Industry ID Extraction**
   ```rust
   // Helper function in auth.rs
   pub fn get_user_industry_id(db: &Connection, token: &str) -> Result<Option<i64>> {
       db.query_row(
           "SELECT u.industry_id FROM sessions s JOIN users u ...",
           |row| row.get::<_, Option<i64>>(0)
       )
   }
   ```

3. **Filtered Queries**
   ```rust
   // In inventory.rs
   let industry_id = get_user_industry_id(&db, &token)?;
   
   let sql = if let Some(ind_id) = industry_id {
       format!("SELECT ... WHERE industry_id = {} ...", ind_id)
   } else {
       "SELECT ... WHERE industry_id IS NULL ...".to_string()
   };
   ```

## Multi-Tenancy Benefits

1. **Data Isolation**: Each user only sees their industry's data
2. **Customization**: Each industry can have unique product attributes
3. **Scalability**: Support for unlimited industries and users
4. **Flexibility**: Easy to add new industries without code changes
5. **Compliance**: Industry-specific rules and regulations can be enforced per industry

## Migration Path

### For Existing Electronics/Lighting Stores
- All existing data is automatically assigned to `electronics` industry
- Users are assigned `electronics` industry_id during migration
- No data loss; backward compatible

### For New Pharmacy Implementations
- Users created for pharmacy get `pharmacy` industry_id
- Pharmacy categories and products are already pre-configured
- Custom attributes like "expiry_date" and "strength" are available

## Frontend Implementation Notes

### 1. Store User's Industry on Login
```javascript
const response = await login(username, password);
localStorage.setItem('userIndustry', JSON.stringify(response.user.industry));
```

### 2. Display Industry-Specific UI
```javascript
const industry = JSON.parse(localStorage.getItem('userIndustry'));
// Show pharmacy-specific UI elements if industry.code === 'pharmacy'
// Show electronics-specific UI if industry.code === 'electronics'
```

### 3. Filter Product Lists
```javascript
const products = await getProducts({
  filter: {
    industry_id: userIndustry.id,  // Optional, backend enforces this anyway
    search: searchTerm
  }
});
```

### 4. Display Industry-Specific Attributes
```javascript
const industry = await getIndustry(industryId);
// Render form fields based on industry.attributes
industry.attributes.forEach(attr => {
  if (attr.attribute_type === 'date') {
    renderDatePicker(attr);
  } else if (attr.attribute_type === 'number') {
    renderNumberInput(attr);
  }
  // etc.
});
```

## Future Enhancements

1. **Industry-Specific Workflows**: Different approval processes for pharmacy vs electronics
2. **Cross-Industry Reporting**: Aggregate reports across all industries for super admins
3. **Industry Templates**: Pre-configured categories and attributes templates per industry
4. **Compliance Modules**: Specific audit trails and compliance features per industry
5. **Dynamic Pricing**: Different pricing rules per industry
6. **Industry-Specific Integrations**: Connect to APIs specific to each industry

## Summary

The multi-industry architecture provides a flexible foundation for serving diverse business types while maintaining data isolation and allowing for deep customization per industry. Each user works within their assigned industry, seeing only relevant products, categories, and settings.
