<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <router-link to="/shop" class="text-2xl font-bold text-blue-600">LumiSync Shop</router-link>
          <router-link to="/shop" class="text-blue-600 hover:text-blue-700">← Back to Shop</router-link>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="isLoading" class="text-center py-12">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
      </div>

      <div v-else-if="error" class="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
        {{ error }}
      </div>

      <div v-else-if="product" class="bg-white rounded-lg border border-gray-200 overflow-hidden shadow-sm">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 p-8">
          <!-- Product Image -->
          <div class="flex items-center justify-center h-96 bg-gradient-to-br from-gray-200 to-gray-300 rounded-lg">
            <span class="text-9xl">💡</span>
          </div>

          <!-- Product Details -->
          <div>
            <h1 class="text-3xl font-bold text-gray-900 mb-2">{{ product.name }}</h1>
            <p class="text-sm text-gray-500 mb-4">SKU: {{ product.sku }}</p>

            <!-- Price and Stock -->
            <div class="mb-6">
              <span class="text-4xl font-bold text-blue-600">₱{{ formatPrice(product.price) }}</span>
              <div class="mt-2">
                <span :class="[
                  'text-sm font-semibold px-3 py-1 rounded inline-block',
                  product.stock > 0 ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
                ]">
                  {{ product.stock > 0 ? `${product.stock} in stock` : 'Out of stock' }}
                </span>
              </div>
            </div>

            <!-- Description -->
            <div class="mb-8">
              <h3 class="text-lg font-semibold text-gray-900 mb-2">Description</h3>
              <p class="text-gray-700">{{ product.description || 'No description available' }}</p>
            </div>

            <!-- Product Info -->
            <div class="mb-8 space-y-2 text-sm">
              <div class="flex items-center gap-2">
                <span class="font-semibold text-gray-700 w-24">Category:</span>
                <span class="text-gray-600">{{ product.category_name || 'N/A' }}</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="font-semibold text-gray-700 w-24">Unit:</span>
                <span class="text-gray-600">{{ product.unit }}</span>
              </div>
            </div>

            <!-- Quantity and Add to Cart -->
            <div v-if="isLoggedIn" class="flex gap-4">
              <div class="flex items-center border border-gray-300 rounded-lg">
                <button
                  @click="decrementQuantity"
                  class="px-3 py-2 text-gray-600 hover:text-gray-900"
                >
                  −
                </button>
                <input
                  v-model.number="quantity"
                  type="number"
                  min="1"
                  :max="product.stock"
                  class="w-16 text-center border-l border-r border-gray-300 py-2 focus:outline-none"
                />
                <button
                  @click="incrementQuantity"
                  class="px-3 py-2 text-gray-600 hover:text-gray-900"
                >
                  +
                </button>
              </div>
              <button
                @click="handleAddToCart"
                :disabled="product.stock <= 0"
                :class="[
                  'flex-1 px-6 py-2 rounded-lg font-semibold text-white',
                  product.stock > 0
                    ? 'bg-blue-600 hover:bg-blue-700'
                    : 'bg-gray-400 cursor-not-allowed'
                ]"
              >
                Add to Cart
              </button>
            </div>
            <button
              v-else
              @click="navigateToLogin"
              class="w-full px-6 py-2 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700"
            >
              Login to Add to Cart
            </button>

            <!-- Continue Shopping -->
            <router-link
              to="/shop"
              class="mt-4 inline-block text-blue-600 hover:text-blue-700 font-medium"
            >
              Continue Shopping
            </router-link>
          </div>
        </div>
      </div>

      <div v-else class="text-center py-12">
        <p class="text-gray-500 text-lg">Product not found</p>
      </div>
    </div>

    <!-- Toast -->
    <transition name="fade">
      <div v-if="successMessage" class="fixed bottom-6 right-6 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg">
        {{ successMessage }}
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useEcommerceStore } from '@/stores/ecommerce'
import { ecommerceProducts } from '@/utils/ecommerce'

const route = useRoute()
const router = useRouter()
const ecommerceStore = useEcommerceStore()

// State
const product = ref(null)
const quantity = ref(1)
const isLoading = ref(false)
const error = ref('')
const successMessage = ref('')

// Computed
const isLoggedIn = computed(() => ecommerceStore.isLoggedIn)

// Methods
function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

function incrementQuantity() {
  if (product.value && quantity.value < product.value.stock) {
    quantity.value++
  }
}

function decrementQuantity() {
  if (quantity.value > 1) {
    quantity.value--
  }
}

function handleAddToCart() {
  if (!isLoggedIn.value) {
    navigateToLogin()
    return
  }

  ecommerceStore.addToCart(product.value, quantity.value)
  successMessage.value = `${product.value.name} added to cart!`
  setTimeout(() => {
    successMessage.value = ''
  }, 2000)
}

function navigateToLogin() {
  router.push('/ecommerce/login')
}

async function loadProduct() {
  isLoading.value = true
  error.value = ''
  try {
    // Fetch all products and find the one with matching ID
    const allProducts = await ecommerceProducts.getAll()
    const found = allProducts.find(p => p.id == route.params.id)
    if (found) {
      product.value = found
      quantity.value = 1
    } else {
      error.value = 'Product not found'
    }
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadProduct()
})
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
