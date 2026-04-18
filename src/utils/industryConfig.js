/**
 * Industry configuration and metadata
 * Maps industry codes to their configuration, features, and custom fields
 */

export const INDUSTRY_CONFIGS = {
  electronics: {
    id: 1,
    code: 'electronics',
    name: 'Electronics & Lighting',
    description: 'Retail of electronic devices and lighting fixtures',
    color: '#3B82F6',
    icon: '💡',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      wattage: { label: 'Wattage (W)', type: 'text', required: false },
      colorTemp: { label: 'Color Temperature', type: 'select', required: false, options: ['Daylight', 'Warm White', 'Cool White', 'RGB'] },
      lumens: { label: 'Brightness (Lumens)', type: 'number', required: false },
      lifespanHours: { label: 'Lifespan (hours)', type: 'number', required: false },
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
    id: 2,
    code: 'pharmacy',
    name: 'Pharmacy & Healthcare',
    description: 'Pharmacy and healthcare products distribution',
    color: '#EC4899',
    icon: '💊',
    features: {
      inventory: true,
      sales: true,
      customers: false, // Uses patients instead
      suppliers: true,
      reports: true,
      pharmacy: true,
      prescriptions: true,
      controlledSubstances: true,
    },
    customFields: {
      strength: { label: 'Strength', type: 'text', required: true },
      dosageForm: { label: 'Dosage Form', type: 'select', required: true, options: ['Tablet', 'Capsule', 'Liquid', 'Injection', 'Cream', 'Ointment', 'Lotion'] },
      manufacturer: { label: 'Manufacturer', type: 'text', required: true },
      expiryDate: { label: 'Expiry Date', type: 'date', required: true },
      genericName: { label: 'Generic Name', type: 'text', required: false },
      ndc: { label: 'NDC Code', type: 'text', required: false },
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
    id: 3,
    code: 'retail',
    name: 'General Retail',
    description: 'General retail and merchandise',
    color: '#8B5CF6',
    icon: '🛍️',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      sku: { label: 'SKU', type: 'text', required: true },
      brand: { label: 'Brand', type: 'text', required: false },
    },
    defaultCategories: ['General', 'Miscellaneous'],
  },

  grocery: {
    id: 4,
    code: 'grocery',
    name: 'Grocery & Food',
    description: 'Grocery stores and food retail',
    color: '#10B981',
    icon: '🛒',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      unitSize: { label: 'Unit Size', type: 'text', required: false },
      expiryDate: { label: 'Best By Date', type: 'date', required: false },
      allergens: { label: 'Allergens', type: 'textarea', required: false },
    },
    defaultCategories: ['Produce', 'Dairy', 'Meat', 'Beverages', 'Pantry', 'Frozen'],
  },

  clothing: {
    id: 5,
    code: 'clothing',
    name: 'Clothing & Fashion',
    description: 'Apparel and fashion retail',
    color: '#F59E0B',
    icon: '👔',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      size: { label: 'Size', type: 'select', required: false, options: ['XS', 'S', 'M', 'L', 'XL', 'XXL'] },
      color: { label: 'Color', type: 'text', required: false },
      material: { label: 'Material', type: 'text', required: false },
      careInstructions: { label: 'Care Instructions', type: 'textarea', required: false },
    },
    defaultCategories: ['Mens', 'Womens', 'Kids', 'Accessories', 'Footwear'],
  },

  furniture: {
    id: 6,
    code: 'furniture',
    name: 'Furniture & Home',
    description: 'Furniture and home goods retail',
    color: '#6366F1',
    icon: '🛋️',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      material: { label: 'Material', type: 'text', required: false },
      dimensions: { label: 'Dimensions', type: 'text', required: false },
      warranty: { label: 'Warranty (months)', type: 'number', required: false },
      color: { label: 'Color', type: 'text', required: false },
    },
    defaultCategories: ['Bedroom', 'Living Room', 'Kitchen', 'Outdoor', 'Decor'],
  },

  automotive: {
    id: 7,
    code: 'automotive',
    name: 'Automotive & Parts',
    description: 'Automotive sales and parts distribution',
    color: '#EF4444',
    icon: '🚗',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      fitment: { label: 'Vehicle Fitment', type: 'text', required: false },
      partNumber: { label: 'Part Number', type: 'text', required: true },
      warranty: { label: 'Warranty (months)', type: 'number', required: false },
    },
    defaultCategories: ['Engine Parts', 'Electrical', 'Suspension', 'Brake System', 'Accessories'],
  },

  cosmetics: {
    id: 8,
    code: 'cosmetics',
    name: 'Cosmetics & Beauty',
    description: 'Beauty products and cosmetics retail',
    color: '#EC4899',
    icon: '💄',
    features: {
      inventory: true,
      sales: true,
      customers: true,
      suppliers: true,
      reports: true,
      pharmacy: false,
      prescriptions: false,
      controlledSubstances: false,
    },
    customFields: {
      skinType: { label: 'Suitable For', type: 'select', required: false, options: ['All', 'Dry', 'Oily', 'Combination', 'Sensitive'] },
      ingredients: { label: 'Key Ingredients', type: 'textarea', required: false },
      volume: { label: 'Volume (ml)', type: 'number', required: false },
      expiryDate: { label: 'Expiry Date', type: 'date', required: false },
    },
    defaultCategories: ['Skincare', 'Makeup', 'Hair Care', 'Fragrance', 'Body Care'],
  },
}

/**
 * Get industry config by ID or code
 */
export function getIndustryConfig(industryIdOrCode) {
  if (typeof industryIdOrCode === 'number') {
    // Search by ID
    return Object.values(INDUSTRY_CONFIGS).find((cfg) => cfg.id === industryIdOrCode)
  }
  // Search by code
  return INDUSTRY_CONFIGS[industryIdOrCode] || null
}

/**
 * Get all industry configs
 */
export function getAllIndustryConfigs() {
  return Object.values(INDUSTRY_CONFIGS)
}

/**
 * Check if industry has a feature
 */
export function hasIndustryFeature(industryCode, featureName) {
  const config = getIndustryConfig(industryCode)
  return config?.features[featureName] === true
}

/**
 * Get custom fields for industry
 */
export function getIndustryCustomFields(industryCode) {
  const config = getIndustryConfig(industryCode)
  return config?.customFields || {}
}

/**
 * Get default categories for industry
 */
export function getIndustryDefaultCategories(industryCode) {
  const config = getIndustryConfig(industryCode)
  return config?.defaultCategories || []
}
