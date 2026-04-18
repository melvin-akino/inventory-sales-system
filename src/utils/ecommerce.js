// E-Commerce API utilities
import { API_BASE_URL } from './api'

const ECOMMERCE_API = `${API_BASE_URL}/ecommerce`
const ADMIN_API = `${API_BASE_URL}/admin`

// ────────────────────────────────────────────────────────────────────────────────
// CUSTOMER AUTH
// ────────────────────────────────────────────────────────────────────────────────

export const ecommerceAuth = {
  async register(email, password, firstName, lastName, phone = '') {
    const response = await fetch(`${ECOMMERCE_API}/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        request: {
          email,
          password,
          first_name: firstName,
          last_name: lastName,
          phone,
        },
      }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Registration failed')
    }
    return await response.json()
  },

  async login(email, password) {
    const response = await fetch(`${ECOMMERCE_API}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        request: { email, password },
      }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Login failed')
    }
    return await response.json()
  },

  async getCustomer(token) {
    const response = await fetch(`${ECOMMERCE_API}/get-customer`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Failed to fetch customer')
    }
    return await response.json()
  },
}

// ────────────────────────────────────────────────────────────────────────────────
// PRODUCTS (Public Catalog)
// ────────────────────────────────────────────────────────────────────────────────

export const ecommerceProducts = {
  async getAll(filter = {}) {
    const response = await fetch(`${ECOMMERCE_API}/products`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ filter }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Failed to fetch products')
    }
    return await response.json()
  },

  async search(query) {
    return this.getAll({ search: query })
  },

  async filterByCategory(categoryId) {
    return this.getAll({ category_id: categoryId })
  },
}

// ────────────────────────────────────────────────────────────────────────────────
// CHECKOUT & ORDERS
// ────────────────────────────────────────────────────────────────────────────────

export const ecommerceCheckout = {
  async createOrder(token, items, shippingAddress, paymentMethod = 'card', discountAmount = 0) {
    const response = await fetch(`${ECOMMERCE_API}/checkout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        token,
        request: {
          items,
          shipping_address: shippingAddress,
          payment_method: paymentMethod,
          discount_amount: discountAmount,
        },
      }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Checkout failed')
    }
    return await response.json()
  },

  async mockPayment(token, orderId, paymentMethod = 'card') {
    const response = await fetch(`${ECOMMERCE_API}/mock-payment`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        token,
        request: {
          order_id: orderId,
          payment_method: paymentMethod,
        },
      }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Payment processing failed')
    }
    return await response.json()
  },
}

// ────────────────────────────────────────────────────────────────────────────────
// ADMIN ORDER MANAGEMENT
// ────────────────────────────────────────────────────────────────────────────────

export const adminOrders = {
  async getAll(token) {
    const response = await fetch(`${ADMIN_API}/orders`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Failed to fetch orders')
    }
    return await response.json()
  },

  async updateStatus(token, orderId, status) {
    const response = await fetch(`${ADMIN_API}/orders/update-status`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        token,
        request: {
          order_id: orderId,
          status,
        },
      }),
    })
    if (!response.ok) {
      const error = await response.json()
      throw new Error(error[1] || 'Failed to update order status')
    }
    return await response.json()
  },
}
