/**
 * Universal API client — bridges Tauri IPC (desktop) and REST HTTP (web).
 * In desktop mode, window.__TAURI__ is defined and we invoke Tauri commands.
 * In web mode, we POST to the backend REST API via /api proxy.
 */

const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined

async function invoke(command, args = {}) {
  if (isTauri) {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/tauri')
    return tauriInvoke(command, args)
  }

  // Web mode: Use /api proxy (Nginx will forward to backend)
  // camelCase command → snake_case HTTP path
  const path = command.replace(/_/g, '-')
  const token = localStorage.getItem('auth_token')
  
  // Ensure path doesn't have leading slash for concatenation
  const pathStr = path.startsWith('/') ? path : `/${path}`
  const url = `/api${pathStr}`
  
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify(args),
  })

  if (!response.ok) {
    const err = await response.text()
    throw new Error(err || `HTTP ${response.status}`)
  }

  return response.json()
}

// Auth
export const authApi = {
  login: (username, password) => invoke('login', { request: { username, password } }),
  logout: (token) => invoke('logout', { token }),
  getCurrentUser: (token) => invoke('get_current_user', { token }),
}

// Users
export const usersApi = {
  getUsers: (token) => invoke('get_users', { token }),
  createUser: (token, request) => invoke('create_user', { token, request }),
  updateUser: (token, id, request) => invoke('update_user', { token, id, request }),
  deleteUser: (token, id) => invoke('delete_user', { token, id }),
  changePassword: (token, currentPassword, newPassword) =>
    invoke('change_password', { token, currentPassword, newPassword }),
}

// Inventory - Categories
export const categoriesApi = {
  getCategories: (token) => invoke('get_categories', { token }),
  createCategory: (token, request) => invoke('create_category', { token, request }),
  updateCategory: (token, id, request) => invoke('update_category', { token, id, request }),
}

// Inventory - Products
export const productsApi = {
  getProducts: (token, filter = null) => invoke('get_products', { token, filter }),
  getProduct: (token, id) => invoke('get_product', { token, id }),
  createProduct: (token, request) => invoke('create_product', { token, request }),
  updateProduct: (token, id, request) => invoke('update_product', { token, id, request }),
  deleteProduct: (token, id) => invoke('delete_product', { token, id }),
  adjustStock: (token, request) => invoke('adjust_stock', { token, request }),
  getStockAdjustments: (token, productId = null) =>
    invoke('get_stock_adjustments', { token, productId }),
}

// Sales
export const salesApi = {
  createSale: (token, request) => invoke('create_sale', { token, request }),
  getSale: (token, id) => invoke('get_sale', { token, id }),
  getSales: (token, filter = null) => invoke('get_sales', { token, filter }),
  voidSale: (token, id, reason) => invoke('void_sale', { token, id, reason }),
}

// Invoices
export const invoicesApi = {
  getInvoices: (token, filter = null) => invoke('get_invoices', { token, filter }),
  getInvoice: (token, id) => invoke('get_invoice', { token, id }),
}

// Customers
export const customersApi = {
  getCustomers: (token, search = null) => invoke('get_customers', { token, search }),
  createCustomer: (token, request) => invoke('create_customer', { token, request }),
  updateCustomer: (token, id, request) => invoke('update_customer', { token, id, request }),
  deleteCustomer: (token, id) => invoke('delete_customer', { token, id }),
}

// Suppliers
export const suppliersApi = {
  getSuppliers: (token, search = null) => invoke('get_suppliers', { token, search }),
  createSupplier: (token, request) => invoke('create_supplier', { token, request }),
  updateSupplier: (token, id, request) => invoke('update_supplier', { token, id, request }),
  deleteSupplier: (token, id) => invoke('delete_supplier', { token, id }),
}

// Reports
export const reportsApi = {
  getDashboardStats: (token) => invoke('get_dashboard_stats', { token }),
  getSalesReport: (token, filter) => invoke('get_sales_report', { token, filter }),
  getInventoryReport: (token) => invoke('get_inventory_report', { token }),
  getProfitLossReport: (token, filter) => invoke('get_profit_loss_report', { token, filter }),
  getVatReport: (token, filter) => invoke('get_vat_report', { token, filter }),
}

// Settings
export const settingsApi = {
  getSettings: (token) => invoke('get_settings', { token }),
  updateSettings: (token, settings) => invoke('update_settings', { token, settings }),
}

// Pharmacy
export const pharmacyApi = {
  // Patients
  getPatients: (token) => invoke('pharmacy/patients', { token }),
  createPatient: (token, request) => invoke('pharmacy/create-patient', { token, request }),
  updatePatient: (token, request) => invoke('pharmacy/update-patient', { token, request }),
  
  // Prescriptions
  getPrescriptions: (token) => invoke('pharmacy/prescriptions', { token }),
  getPrescriptionByNumber: (token, prescription_number) => invoke('pharmacy/prescriptions/by-number', { token, prescription_number }),
  createPrescription: (token, request) => invoke('pharmacy/create-prescription', { token, request }),
  
  // Controlled Substances
  logControlledSubstance: (token, request) => invoke('pharmacy/controlled-log', { token, request }),
  getControlledLogs: (token) => invoke('pharmacy/controlled-logs', { token }),
  
  // Alerts
  getExpiryAlerts: (token) => invoke('pharmacy/expiry-alerts', { token }),
}

// Industries
export const industriesApi = {
  getIndustries: (token) => invoke('get_industries', { token }),
  getIndustry: (token, id) => invoke('get_industry', { token, id }),
  addIndustryAttribute: (token, request) => invoke('add_industry_attribute', { token, request }),
  assignUserToIndustry: (token, request) => invoke('assign_user_to_industry', { token, request }),
}
