/**
 * Utility helpers — formatting for the Philippine market.
 */

export const PHP_SYMBOL = '₱'

export function formatCurrency(amount, symbol = PHP_SYMBOL) {
  if (amount === null || amount === undefined || isNaN(amount)) return `${symbol}0.00`
  return `${symbol}${Number(amount).toLocaleString('en-PH', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  })}`
}

export function formatNumber(n) {
  return Number(n).toLocaleString('en-PH')
}

export function formatDate(dateStr) {
  if (!dateStr) return '—'
  const d = new Date(dateStr)
  if (isNaN(d)) return dateStr
  return d.toLocaleDateString('en-PH', { year: 'numeric', month: 'short', day: 'numeric' })
}

export function formatDateTime(dateStr) {
  if (!dateStr) return '—'
  const d = new Date(dateStr)
  if (isNaN(d)) return dateStr
  return d.toLocaleString('en-PH', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

export function todayISO() {
  return new Date().toISOString().split('T')[0]
}

export function firstDayOfMonth() {
  const d = new Date()
  return new Date(d.getFullYear(), d.getMonth(), 1).toISOString().split('T')[0]
}

export const PAYMENT_METHODS = [
  { value: 'cash', label: 'Cash' },
  { value: 'card', label: 'Credit/Debit Card' },
  { value: 'gcash', label: 'GCash' },
  { value: 'bank_transfer', label: 'Bank Transfer' },
  { value: 'credit', label: 'Store Credit' },
]

export const ROLES = [
  { value: 'super_admin', label: 'Super Admin', color: 'red' },
  { value: 'admin', label: 'Admin', color: 'orange' },
  { value: 'manager', label: 'Manager', color: 'blue' },
  { value: 'cashier', label: 'Cashier', color: 'green' },
  { value: 'viewer', label: 'Viewer', color: 'gray' },
]

export function roleLabel(role) {
  return ROLES.find((r) => r.value === role)?.label ?? role
}

export function roleColor(role) {
  const colors = {
    super_admin: 'bg-red-100 text-red-800',
    admin: 'bg-orange-100 text-orange-800',
    manager: 'bg-blue-100 text-blue-800',
    cashier: 'bg-green-100 text-green-800',
    viewer: 'bg-gray-100 text-gray-700',
  }
  return colors[role] ?? 'bg-gray-100 text-gray-700'
}

export function stockStatusColor(qty, reorder) {
  if (qty === 0) return 'bg-red-100 text-red-800'
  if (qty <= reorder) return 'bg-yellow-100 text-yellow-800'
  return 'bg-green-100 text-green-800'
}

export function stockStatusLabel(qty, reorder) {
  if (qty === 0) return 'Out of Stock'
  if (qty <= reorder) return 'Low Stock'
  return 'In Stock'
}

export function paymentLabel(method) {
  return PAYMENT_METHODS.find((m) => m.value === method)?.label ?? method
}

export function canAccess(userRole, allowedRoles) {
  return allowedRoles.includes(userRole)
}

export const ACCESS = {
  FULL: ['super_admin', 'admin', 'manager', 'cashier', 'viewer'],
  MANAGE: ['super_admin', 'admin', 'manager'],
  ADMIN: ['super_admin', 'admin'],
  SUPER: ['super_admin'],
  SALES: ['super_admin', 'admin', 'manager', 'cashier'],
}
