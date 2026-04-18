import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { ecommerceAuth, ecommerceProducts, ecommerceCheckout } from '@/utils/ecommerce'

export const useEcommerceStore = defineStore('ecommerce', () => {
  // ── State ──────────────────────────────────────────────────────────────────────
  const customer = ref(null)
  const token = ref(localStorage.getItem('ecommerce_token') || '')
  const isLoggedIn = ref(!!token.value)
  const cartItems = ref([])
  const currentOrder = ref(null)
  const isLoading = ref(false)
  const error = ref('')

  // ── Computed ───────────────────────────────────────────────────────────────────
  const cartTotal = computed(() => {
    return cartItems.value.reduce((sum, item) => sum + item.price * item.quantity, 0)
  })

  const cartItemCount = computed(() => {
    return cartItems.value.reduce((count, item) => count + item.quantity, 0)
  })

  // ── Authentication ─────────────────────────────────────────────────────────────
  async function register(email, password, firstName, lastName, phone = '') {
    isLoading.value = true
    error.value = ''
    try {
      const result = await ecommerceAuth.register(email, password, firstName, lastName, phone)
      customer.value = result
      return result
    } catch (err) {
      error.value = err.message
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function login(email, password) {
    isLoading.value = true
    error.value = ''
    try {
      const result = await ecommerceAuth.login(email, password)
      token.value = result.token
      customer.value = result.customer
      isLoggedIn.value = true
      localStorage.setItem('ecommerce_token', result.token)
      return result
    } catch (err) {
      error.value = err.message
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function logout() {
    customer.value = null
    token.value = ''
    isLoggedIn.value = false
    cartItems.value = []
    currentOrder.value = null
    localStorage.removeItem('ecommerce_token')
  }

  async function getCustomer() {
    if (!token.value) {
      throw new Error('Not logged in')
    }
    isLoading.value = true
    error.value = ''
    try {
      const result = await ecommerceAuth.getCustomer(token.value)
      customer.value = result
      return result
    } catch (err) {
      error.value = err.message
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // ── Cart Management ────────────────────────────────────────────────────────────
  function addToCart(product, quantity = 1) {
    const existing = cartItems.value.find(item => item.id === product.id)
    if (existing) {
      existing.quantity += quantity
    } else {
      cartItems.value.push({
        id: product.id,
        name: product.name,
        price: product.price,
        stock: product.stock,
        quantity: Math.min(quantity, product.stock),
      })
    }
  }

  function removeFromCart(productId) {
    const index = cartItems.value.findIndex(item => item.id === productId)
    if (index > -1) {
      cartItems.value.splice(index, 1)
    }
  }

  function updateQuantity(productId, quantity) {
    const item = cartItems.value.find(item => item.id === productId)
    if (item) {
      item.quantity = Math.max(1, Math.min(quantity, item.stock))
    }
  }

  function clearCart() {
    cartItems.value = []
  }

  // ── Checkout ───────────────────────────────────────────────────────────────────
  async function checkout(shippingAddress, paymentMethod = 'card', discountAmount = 0) {
    if (!token.value) {
      throw new Error('Not logged in')
    }
    if (cartItems.value.length === 0) {
      throw new Error('Cart is empty')
    }

    isLoading.value = true
    error.value = ''
    try {
      const items = cartItems.value.map(item => ({
        product_id: item.id,
        quantity: item.quantity,
      }))

      const order = await ecommerceCheckout.createOrder(
        token.value,
        items,
        shippingAddress,
        paymentMethod,
        discountAmount
      )

      currentOrder.value = order
      clearCart()
      return order
    } catch (err) {
      error.value = err.message
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function processPayment(paymentMethod = 'card') {
    if (!token.value || !currentOrder.value) {
      throw new Error('Invalid payment state')
    }

    isLoading.value = true
    error.value = ''
    try {
      const result = await ecommerceCheckout.mockPayment(
        token.value,
        currentOrder.value.id,
        paymentMethod
      )

      currentOrder.value = result.order
      return result
    } catch (err) {
      error.value = err.message
      throw err
    } finally {
      isLoading.value = false
    }
  }

  return {
    // State
    customer,
    token,
    isLoggedIn,
    cartItems,
    currentOrder,
    isLoading,
    error,
    // Computed
    cartTotal,
    cartItemCount,
    // Methods
    register,
    login,
    logout,
    getCustomer,
    addToCart,
    removeFromCart,
    updateQuantity,
    clearCart,
    checkout,
    processPayment,
  }
})
