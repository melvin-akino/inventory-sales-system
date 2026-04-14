// Industry Configuration System
// Defines what each industry is and its features

export const INDUSTRIES = {
  ELECTRICAL: 'electrical',
  PHARMACY: 'pharmacy',
  RETAIL: 'retail',
  RESTAURANT: 'restaurant',
  SUPERMARKET: 'supermarket',
}

export const INDUSTRY_CONFIG = {
  electrical: {
    id: 'electrical',
    name: 'Electrical Supply Store',
    icon: '🔌',
    color: '#3b82f6',
    description: 'Electrical equipment and lighting supplies',
    features: {
      pos: true,
      inventory: true,
      customers: true,
      suppliers: true,
      sales: true,
      reports: true,
    },
    customFields: {
      // Electrical-specific product fields
      voltage: 'Voltage (V)',
      wattage: 'Wattage (W)',
      warranty: 'Warranty (months)',
    },
    defaultCategories: [
      'LED Bulbs',
      'Fluorescent',
      'Downlights',
      'Streetlights',
      'Floodlights',
      'Decorative',
      'Accessories',
    ],
  },

  pharmacy: {
    id: 'pharmacy',
    name: 'Pharmacy',
    icon: '💊',
    color: '#10b981',
    description: 'Pharmacy medication and health products',
    features: {
      pos: true,
      inventory: true,
      customers: true,
      suppliers: true,
      sales: true,
      reports: true,
      prescriptions: true, // Pharmacy-specific
      patients: true, // Pharmacy-specific
      expiry_tracking: true, // Pharmacy-specific
      controlled_substances: true, // Pharmacy-specific
      insurance: true, // Pharmacy-specific
    },
    customFields: {
      batch_number: 'Batch Number',
      expiry_date: 'Expiry Date',
      generic_name: 'Generic Name',
      strength: 'Strength/Dosage',
      manufacturer: 'Manufacturer',
      prescription_required: 'Requires Prescription',
    },
    defaultCategories: [
      'Antibiotics',
      'Painkillers',
      'Cough & Cold',
      'Digestive',
      'Vitamins & Supplements',
      'Topical',
      'First Aid',
      'Over-the-Counter',
    ],
  },

  retail: {
    id: 'retail',
    name: 'Retail Store',
    icon: '🛍️',
    color: '#f59e0b',
    description: 'General retail and fashion store',
    features: {
      pos: true,
      inventory: true,
      customers: true,
      suppliers: true,
      sales: true,
      reports: true,
      promotions: true,
      loyalty: true,
    },
    customFields: {
      size: 'Size',
      color: 'Color',
      material: 'Material',
      brand: 'Brand',
    },
    defaultCategories: [
      'Clothing',
      'Footwear',
      'Accessories',
      'Electronics',
      'Home & Garden',
    ],
  },

  restaurant: {
    id: 'restaurant',
    name: 'Restaurant/Cafe',
    icon: '🍽️',
    color: '#ef4444',
    description: 'Food and beverage service',
    features: {
      pos: true,
      inventory: true,
      customers: true,
      suppliers: true,
      sales: true,
      reports: true,
      menu: true,
      orders: true,
      kitchen_display: true,
      reservations: true,
    },
    customFields: {
      recipe: 'Recipe/Ingredients',
      preparation_time: 'Prep Time (min)',
      allergens: 'Allergens',
      calories: 'Calories',
    },
    defaultCategories: [
      'Appetizers',
      'Main Course',
      'Desserts',
      'Beverages',
      'Sides',
    ],
  },

  supermarket: {
    id: 'supermarket',
    name: 'Supermarket',
    icon: '🏪',
    color: '#8b5cf6',
    description: 'Large-scale retail with groceries',
    features: {
      pos: true,
      inventory: true,
      customers: true,
      suppliers: true,
      sales: true,
      reports: true,
      promotions: true,
      loyalty: true,
      bulk_operations: true,
    },
    customFields: {
      barcode: 'Barcode',
      weight: 'Weight',
      expiry_date: 'Expiry Date',
      origin: 'Country of Origin',
    },
    defaultCategories: [
      'Groceries',
      'Fruits & Vegetables',
      'Meat & Seafood',
      'Dairy',
      'Beverages',
      'Snacks',
      'Personal Care',
      'Household',
    ],
  },
}

/**
 * Get industry configuration by ID
 */
export function getIndustryConfig(industryId) {
  return INDUSTRY_CONFIG[industryId] || INDUSTRY_CONFIG.electrical
}

/**
 * Get list of all available industries
 */
export function getAvailableIndustries() {
  return Object.values(INDUSTRY_CONFIG)
}

/**
 * Check if industry has a specific feature
 */
export function hasFeature(industryId, featureName) {
  const config = getIndustryConfig(industryId)
  return config.features[featureName] === true
}
