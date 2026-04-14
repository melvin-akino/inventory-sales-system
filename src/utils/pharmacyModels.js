// Pharmacy-Specific Models and Features

export const PHARMACY_FEATURES = {
  PRESCRIPTIONS: 'prescriptions',
  PATIENTS: 'patients',
  EXPIRY_TRACKING: 'expiry_tracking',
  CONTROLLED_SUBSTANCES: 'controlled_substances',
  INSURANCE: 'insurance',
  REFILLS: 'refills',
}

/**
 * Prescription object structure
 */
export const PRESCRIPTION_SCHEMA = {
  id: 'auto',
  prescription_number: 'string',
  patient_id: 'integer',
  doctor_name: 'string',
  doctor_license: 'string',
  medications: 'array<medication>',
  prescribing_date: 'date',
  expiry_date: 'date',
  instructions: 'text',
  refills_allowed: 'integer',
  refills_used: 'integer',
  is_controlled: 'boolean',
  status: 'enum(active, filled, expired, voided)',
  notes: 'text',
  created_at: 'datetime',
  updated_at: 'datetime',
}

/**
 * Patient object structure (pharmacy-specific)
 */
export const PATIENT_SCHEMA = {
  id: 'auto',
  name: 'string',
  phone: 'string',
  email: 'string',
  date_of_birth: 'date',
  gender: 'enum(M, F, Other)',
  address: 'text',
  allergies: 'text',
  medical_conditions: 'text',
  insurance_provider: 'string',
  insurance_member_id: 'string',
  preferred_pharmacy: 'string',
  emergency_contact: 'string',
  emergency_contact_phone: 'string',
  is_active: 'boolean',
  created_at: 'datetime',
  updated_at: 'datetime',
}

/**
 * Pharmacy Product (extends base product)
 */
export const PHARMACY_PRODUCT_SCHEMA = {
  // Base fields
  id: 'auto',
  category_id: 'integer',
  sku: 'string',
  name: 'string',
  description: 'text',
  
  // Pharmacy-specific fields
  generic_name: 'string',
  brand_name: 'string',
  strength: 'string', // e.g., "500mg", "10ml"
  dosage_form: 'enum(tablet, capsule, liquid, injection, cream, ointment, spray, other)',
  manufacturer: 'string',
  
  // Inventory
  cost_price: 'decimal',
  selling_price: 'decimal',
  quantity: 'integer',
  reorder_level: 'integer',
  
  // Pharmacy tracking
  batch_number: 'string',
  expiry_date: 'date',
  manufactured_date: 'date',
  storage_condition: 'string', // e.g., "Room temperature", "Refrigerated"
  
  // Regulatory
  requires_prescription: 'boolean',
  is_controlled_substance: 'boolean',
  controlled_category: 'enum(schedule1, schedule2, schedule3, schedule4, schedule5, none)',
  dea_number: 'string',
  
  // Insurance
  is_covered_by_insurance: 'boolean',
  insurance_tier: 'integer', // 1=copay, 2=moderate, 3=higher
  ndc_code: 'string', // National Drug Code
  
  is_active: 'boolean',
  created_at: 'datetime',
  updated_at: 'datetime',
}

/**
 * Refill request object
 */
export const REFILL_SCHEMA = {
  id: 'auto',
  prescription_id: 'integer',
  patient_id: 'integer',
  product_id: 'integer',
  refill_number: 'integer',
  requested_date: 'date',
  filled_date: 'date',
  quantity: 'integer',
  status: 'enum(pending, filled, denied, expired)',
  reason: 'text',
  created_at: 'datetime',
}

/**
 * Pharmacy Sale (extends base sale)
 * Includes prescription tracking
 */
export const PHARMACY_SALE_SCHEMA = {
  // Base fields
  id: 'auto',
  sale_number: 'string',
  customer_id: 'integer',
  user_id: 'integer',
  
  // Pharmacy-specific
  patient_id: 'integer', // Link to patient
  prescription_id: 'integer', // Link to prescription
  pharmacist_name: 'string',
  pharmacist_license: 'string',
  
  // Sale details
  sale_date: 'datetime',
  items: 'array<sale_item>',
  
  // Pricing
  subtotal: 'decimal',
  discount_amount: 'decimal',
  vat_amount: 'decimal',
  total_amount: 'decimal',
  insurance_covered: 'decimal',
  patient_paid: 'decimal',
  
  // Insurance
  insurance_claim_number: 'string',
  insurance_status: 'enum(pending, approved, denied)',
  
  payment_method: 'enum(cash, card, insurance, mixed)',
  amount_paid: 'decimal',
  change_amount: 'decimal',
  
  status: 'enum(completed, void, returned)',
  notes: 'text',
  created_at: 'datetime',
}

/**
 * Pharmacy Inventory Alert
 */
export const INVENTORY_ALERT_SCHEMA = {
  id: 'auto',
  product_id: 'integer',
  alert_type: 'enum(low_stock, expiry_near, expired, out_of_stock)',
  alert_message: 'text',
  days_until_expiry: 'integer',
  quantity_threshold: 'integer',
  is_resolved: 'boolean',
  resolved_by: 'integer', // user_id
  resolved_at: 'datetime',
  created_at: 'datetime',
}

/**
 * Controlled Substance Log
 * For regulatory compliance and audit trail
 */
export const CONTROLLED_SUBSTANCE_LOG = {
  id: 'auto',
  product_id: 'integer',
  product_name: 'string',
  transaction_type: 'enum(sale, return, adjustment, destruction)',
  quantity: 'integer',
  user_id: 'integer',
  patient_id: 'integer',
  prescription_id: 'integer',
  witness_name: 'string', // Required for controlled substances
  reason: 'text',
  created_at: 'datetime',
}
