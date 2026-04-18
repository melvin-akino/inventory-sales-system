<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200 sticky top-0 z-40">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <router-link to="/shop" class="text-2xl font-bold text-blue-600">LumiSync Shop</router-link>
          <div class="flex items-center gap-4">
            <router-link to="/shop" class="text-gray-700 hover:text-gray-900">Products</router-link>
            <router-link v-if="isLoggedIn" to="/cart" class="relative">
              <button class="text-gray-700 hover:text-gray-900 flex items-center gap-1">
                🛒 Cart
                <span v-if="cartItemCount > 0" class="bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                  {{ cartItemCount }}
                </span>
              </button>
            </router-link>
            <div class="flex items-center gap-2">
              <span v-if="isLoggedIn" class="text-sm text-gray-700">{{ customer?.first_name }}</span>
              <button
                v-if="isLoggedIn"
                @click="handleLogout"
                class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600"
              >
                Logout
              </button>
              <router-link v-else to="/ecommerce/login" class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600">
                Login
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- Filters -->
      <div class="mb-8">
        <div class="flex gap-4">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search products..."
            class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 flex-1"
            @input="handleSearch"
          />
          <select
            v-model="selectedCategory"
            class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            @change="handleCategoryFilter"
          >
            <option value="">All Categories</option>
            <option value="1">LED Bulbs</option>
            <option value="2">Fluorescent</option>
            <option value="3">Downlights</option>
            <option value="4">Streetlights</option>
            <option value="5">Floodlights</option>
            <option value="6">Decorative</option>
            <option value="7">Accessories</option>
          </select>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="text-center py-12">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
        <p class="mt-2 text-gray-600">Loading products...</p>
      </div>

      <!-- Error State -->
      <div v-if="error" class="bg-red-50 border border-red-200 rounded-lg p-4 mb-4 text-red-700">
        {{ error }}
      </div>

      <!-- Products Grid -->
      <div v-if="filteredProducts.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        <div
          v-for="product in filteredProducts"
          :key="product.id"
          class="bg-white rounded-lg border border-gray-200 overflow-hidden hover:shadow-lg transition-shadow"
        >
          <!-- Product Image -->
          <div class="w-full h-48 bg-gradient-to-br from-gray-200 to-gray-300 flex items-center justify-center">
            <span class="text-5xl">💡</span>
          </div>

          <!-- Product Info -->
          <div class="p-4">
            <h3 class="font-semibold text-gray-900 mb-1">{{ product.name }}</h3>
            <p class="text-xs text-gray-500 mb-2">{{ product.sku }}</p>
            <p class="text-sm text-gray-600 mb-3 line-clamp-2">{{ product.description }}</p>

            <!-- Price and Stock -->
            <div class="flex items-center justify-between mb-4">
              <span class="text-xl font-bold text-blue-600">₱{{ formatPrice(product.price) }}</span>
              <span :class="[
                'text-xs font-semibold px-2 py-1 rounded',
                product.stock > 0 ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
              ]">
                {{ product.stock > 0 ? `${product.stock} in stock` : 'Out of stock' }}
              </span>
            </div>

            <!-- Actions -->
            <div class="flex gap-2">
              <router-link
                :to="`/product/${product.id}`"
                class="flex-1 px-3 py-2 bg-gray-200 text-gray-900 rounded hover:bg-gray-300 text-center text-sm font-medium"
              >
                View
              </router-link>
              <button
                v-if="isLoggedIn"
                @click="handleAddToCart(product)"
                :disabled="product.stock <= 0"
                :class="[
                  'flex-1 px-3 py-2 rounded text-sm font-medium',
                  product.stock > 0
                    ? 'bg-blue-500 text-white hover:bg-blue-600'
                    : 'bg-gray-300 text-gray-500 cursor-not-allowed'
                ]"
              >
                Add to Cart
              </button>
              <button
                v-else
                @click="navigateToLogin"
                class="flex-1 px-3 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 text-sm font-medium"
              >
                Add to Cart
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-12">
        <p class="text-gray-500 text-lg mb-2">No products found</p>
        <p class="text-sm text-gray-400">Try adjusting your search or filters</p>
      </div>
    </div>

    <!-- Toast Notification -->
    <transition name="fade">
      <div v-if="successMessage" class="fixed bottom-6 right-6 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg">
        {{ successMessage }}
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useEcommerceStore } from '@/stores/ecommerce'
import { ecommerceProducts } from '@/utils/ecommerce'

const router = useRouter()
const ecommerceStore = useEcommerceStore()

// State
const products = ref([])
const searchQuery = ref('')
const selectedCategory = ref('')
const isLoading = ref(false)
const error = ref('')
const successMessage = ref('')

// Computed
const isLoggedIn = computed(() => ecommerceStore.isLoggedIn)
const customer = computed(() => ecommerceStore.customer)
const cartItemCount = computed(() => ecommerceStore.cartItemCount)

const filteredProducts = computed(() => {
  let filtered = products.value

  if (selectedCategory.value) {
    filtered = filtered.filter(p => p.category_id == selectedCategory.value)
  }

  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    filtered = filtered.filter(p =>
      p.name.toLowerCase().includes(q) ||
      p.sku.toLowerCase().includes(q) ||
      (p.description?.toLowerCase() || '').includes(q)
    )
  }

  return filtered
})

// Methods
function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

function handleAddToCart(product) {
  if (!isLoggedIn.value) {
    navigateToLogin()
    return
  }
  ecommerceStore.addToCart(product)
  successMessage.value = `${product.name} added to cart!`
  setTimeout(() => {
    successMessage.value = ''
  }, 2000)
}

function navigateToLogin() {
  router.push('/ecommerce/login')
}

function handleLogout() {
  ecommerceStore.logout()
  router.push('/ecommerce/login')
}

async function handleSearch() {
  if (!searchQuery.value.trim()) {
    loadProducts()
    return
  }
  isLoading.value = true
  try {
    const result = await ecommerceProducts.search(searchQuery.value)
    products.value = result || []
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

async function handleCategoryFilter() {
  if (!selectedCategory.value) {
    loadProducts()
    return
  }
  isLoading.value = true
  try {
    const result = await ecommerceProducts.filterByCategory(parseInt(selectedCategory.value))
    products.value = result || []
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

async function loadProducts() {
  isLoading.value = true
  error.value = ''
  try {
    const result = await ecommerceProducts.getAll()
    products.value = result || []
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadProducts()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
