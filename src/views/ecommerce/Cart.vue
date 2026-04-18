<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <router-link to="/shop" class="text-2xl font-bold text-blue-600">LumiSync Shop</router-link>
          <router-link to="/shop" class="text-blue-600 hover:text-blue-700">← Continue Shopping</router-link>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="cartItems.length === 0" class="text-center py-12">
        <p class="text-4xl mb-4">🛒</p>
        <p class="text-xl text-gray-600 mb-4">Your cart is empty</p>
        <router-link to="/shop" class="inline-block px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
          Start Shopping
        </router-link>
      </div>

      <div v-else>
        <!-- Cart Items -->
        <div class="bg-white rounded-lg border border-gray-200 overflow-hidden mb-8">
          <div class="p-6 border-b border-gray-200">
            <h2 class="text-2xl font-bold text-gray-900">Shopping Cart</h2>
            <p class="text-sm text-gray-600 mt-1">{{ cartItems.length }} item(s)</p>
          </div>

          <div class="divide-y divide-gray-200">
            <div
              v-for="(item, idx) in cartItems"
              :key="idx"
              class="p-6 flex gap-6"
            >
              <!-- Product Icon -->
              <div class="flex-shrink-0 w-20 h-20 bg-gradient-to-br from-gray-200 to-gray-300 rounded-lg flex items-center justify-center">
                <span class="text-3xl">💡</span>
              </div>

              <!-- Product Details -->
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-gray-900">{{ item.name }}</h3>
                <p class="text-gray-600">₱{{ formatPrice(item.price) }} each</p>
              </div>

              <!-- Quantity Controls -->
              <div class="flex items-center gap-3">
                <div class="flex items-center border border-gray-300 rounded-lg">
                  <button
                    @click="ecommerceStore.updateQuantity(item.id, item.quantity - 1)"
                    class="px-2 py-1 text-gray-600 hover:text-gray-900"
                  >
                    −
                  </button>
                  <span class="px-4 py-1 border-l border-r border-gray-300">{{ item.quantity }}</span>
                  <button
                    @click="ecommerceStore.updateQuantity(item.id, item.quantity + 1)"
                    :disabled="item.quantity >= item.stock"
                    :class="[
                      'px-2 py-1',
                      item.quantity >= item.stock
                        ? 'text-gray-300 cursor-not-allowed'
                        : 'text-gray-600 hover:text-gray-900'
                    ]"
                  >
                    +
                  </button>
                </div>
              </div>

              <!-- Item Total -->
              <div class="text-right w-24">
                <p class="text-lg font-semibold text-gray-900">
                  ₱{{ formatPrice(item.price * item.quantity) }}
                </p>
                <button
                  @click="ecommerceStore.removeFromCart(item.id)"
                  class="text-sm text-red-600 hover:text-red-700 mt-2"
                >
                  Remove
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Order Summary -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          <!-- Checkout Form -->
          <div class="md:col-span-2">
            <form @submit.prevent="handleCheckout" class="bg-white rounded-lg border border-gray-200 p-6">
              <h3 class="text-lg font-semibold text-gray-900 mb-4">Shipping Address</h3>

              <div class="mb-4">
                <label class="block text-sm font-medium text-gray-700 mb-1">Address</label>
                <input
                  v-model="shippingAddress"
                  type="text"
                  placeholder="123 Main Street"
                  required
                  class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div class="mb-4">
                <label class="block text-sm font-medium text-gray-700 mb-1">City</label>
                <input
                  v-model="shippingCity"
                  type="text"
                  placeholder="Manila"
                  class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div class="grid grid-cols-2 gap-4 mb-6">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">State/Province</label>
                  <input
                    v-model="shippingState"
                    type="text"
                    placeholder="NCR"
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">ZIP Code</label>
                  <input
                    v-model="shippingZip"
                    type="text"
                    placeholder="1000"
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
              </div>

              <div class="mb-6">
                <label class="block text-sm font-medium text-gray-700 mb-1">Payment Method</label>
                <select
                  v-model="paymentMethod"
                  class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="card">💳 Credit/Debit Card</option>
                  <option value="gcash">📱 GCash</option>
                  <option value="maya">📱 Maya</option>
                  <option value="bank_transfer">🏦 Bank Transfer</option>
                </select>
              </div>

              <button
                type="submit"
                :disabled="isLoading"
                :class="[
                  'w-full px-6 py-3 rounded-lg font-semibold text-white',
                  isLoading
                    ? 'bg-gray-400 cursor-not-allowed'
                    : 'bg-blue-600 hover:bg-blue-700'
                ]"
              >
                {{ isLoading ? 'Processing...' : 'Proceed to Payment' }}
              </button>
            </form>
          </div>

          <!-- Order Summary Sidebar -->
          <div class="bg-white rounded-lg border border-gray-200 p-6 h-fit sticky top-20">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Order Summary</h3>

            <div class="space-y-3 mb-4">
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Subtotal:</span>
                <span class="font-semibold text-gray-900">₱{{ formatPrice(subtotal) }}</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Shipping:</span>
                <span class="font-semibold text-gray-900">₱0.00</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Tax (12%):</span>
                <span class="font-semibold text-gray-900">₱{{ formatPrice(tax) }}</span>
              </div>
            </div>

            <div class="border-t border-gray-200 pt-4">
              <div class="flex justify-between">
                <span class="font-semibold text-gray-900">Total:</span>
                <span class="text-2xl font-bold text-blue-600">₱{{ formatPrice(total) }}</span>
              </div>
            </div>

            <button
              @click="ecommerceStore.clearCart"
              class="w-full mt-6 px-4 py-2 border border-red-300 text-red-600 rounded-lg hover:bg-red-50"
            >
              Clear Cart
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Error Toast -->
    <transition name="fade">
      <div v-if="error" class="fixed bottom-6 right-6 bg-red-500 text-white px-6 py-3 rounded-lg shadow-lg">
        {{ error }}
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useEcommerceStore } from '@/stores/ecommerce'

const router = useRouter()
const ecommerceStore = useEcommerceStore()

// State
const shippingAddress = ref('')
const shippingCity = ref('')
const shippingState = ref('')
const shippingZip = ref('')
const paymentMethod = ref('card')
const isLoading = ref(false)
const error = ref('')

// Computed
const cartItems = computed(() => ecommerceStore.cartItems)
const subtotal = computed(() => ecommerceStore.cartTotal)
const tax = computed(() => subtotal.value * 0.12)
const total = computed(() => subtotal.value + tax.value)

// Methods
function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

async function handleCheckout() {
  if (!shippingAddress.value.trim()) {
    error.value = 'Please enter a shipping address'
    return
  }

  isLoading.value = true
  error.value = ''

  try {
    const address = `${shippingAddress.value}, ${shippingCity.value}, ${shippingState.value} ${shippingZip.value}`
    await ecommerceStore.checkout(address, paymentMethod.value)
    router.push('/checkout/payment')
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
