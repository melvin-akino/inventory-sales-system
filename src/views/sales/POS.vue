<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Top Header -->
    <div class="sticky top-0 z-40 bg-white border-b border-gray-200 shadow-sm">
      <div class="px-6 py-4 flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Point of Sale</h1>
          <p class="text-sm text-gray-500">{{ new Date().toLocaleDateString() }}</p>
        </div>
        <div class="flex items-center gap-4">
          <div class="text-right">
            <p class="text-sm font-semibold text-gray-900">{{ authStore.user?.username || 'User' }}</p>
            <p class="text-xs text-gray-500">{{ authStore.user?.role || 'Cashier' }}</p>
          </div>
          <img :src="userAvatar" class="w-10 h-10 rounded-full bg-primary-500" />
        </div>
      </div>
    </div>

    <div class="flex h-[calc(100vh-90px)]">
      <!-- Left Sidebar - Categories & Filters -->
      <div class="w-64 bg-white border-r border-gray-200 overflow-y-auto">
        <div class="p-4">
          <!-- Search Bar -->
          <div class="mb-4">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search products..."
              class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <!-- Categories -->
          <div class="mb-6">
            <h3 class="text-sm font-semibold text-gray-900 mb-3">Categories</h3>
            <button
              @click="selectedCategory = null"
              :class="[
                'w-full text-left px-3 py-2 rounded-lg text-sm mb-2 transition-colors',
                selectedCategory === null
                  ? 'bg-primary-100 text-primary-900 font-medium'
                  : 'text-gray-700 hover:bg-gray-100'
              ]"
            >
              All Products
            </button>
            <button
              v-for="cat in categories"
              :key="cat.id"
              @click="selectedCategory = cat.id"
              :class="[
                'w-full text-left px-3 py-2 rounded-lg text-sm mb-2 transition-colors',
                selectedCategory === cat.id
                  ? 'bg-primary-100 text-primary-900 font-medium'
                  : 'text-gray-700 hover:bg-gray-100'
              ]"
            >
              {{ cat.name }}
            </button>
          </div>

          <!-- Quick Actions -->
          <div class="border-t pt-4">
            <h3 class="text-sm font-semibold text-gray-900 mb-3">Quick Actions</h3>
            <button
              @click="clearCart"
              class="w-full px-3 py-2 rounded-lg text-sm text-red-600 hover:bg-red-50 mb-2 transition-colors"
            >
              Clear Cart
            </button>
            <button
              @click="showCustomer = !showCustomer"
              class="w-full px-3 py-2 rounded-lg text-sm text-blue-600 hover:bg-blue-50 transition-colors"
            >
              {{ showCustomer ? 'Hide' : 'Add' }} Customer
            </button>
          </div>
        </div>
      </div>

      <!-- Main Content - Products Grid -->
      <div class="flex-1 overflow-y-auto">
        <div class="p-6">
          <!-- Customer Selection (if toggled) -->
          <div v-if="showCustomer" class="mb-6 p-4 bg-blue-50 rounded-lg border border-blue-200">
            <div class="flex items-center gap-4">
              <div class="flex-1">
                <label class="text-sm font-medium text-gray-700 block mb-1">Customer (Optional)</label>
                <input
                  v-model="selectedCustomer"
                  type="text"
                  placeholder="Search or add new customer..."
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>
          </div>

          <!-- Products Grid -->
          <div v-if="filteredProducts.length > 0" class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
            <button
              v-for="product in filteredProducts"
              :key="product.id"
              @click="addToCart(product)"
              class="bg-white rounded-xl border-2 border-gray-200 p-3 hover:border-primary-500 hover:shadow-lg transition-all duration-200 text-left"
            >
              <!-- Product Image Placeholder -->
              <div class="w-full aspect-square bg-gradient-to-br from-gray-200 to-gray-300 rounded-lg mb-2 flex items-center justify-center">
                <span class="text-4xl">📦</span>
              </div>
              
              <!-- Product Info -->
              <h4 class="font-semibold text-gray-900 text-sm mb-1 line-clamp-2">{{ product.name }}</h4>
              <p class="text-xs text-gray-500 mb-2">{{ product.sku }}</p>
              
              <!-- Price -->
              <p class="text-lg font-bold text-primary-600 mb-2">₱{{ formatPrice(product.selling_price) }}</p>
              
              <!-- Stock -->
              <p :class="[
                'text-xs font-medium',
                product.quantity_on_hand > 0 ? 'text-green-600' : 'text-red-600'
              ]">
                {{ product.quantity_on_hand > 0 ? `${product.quantity_on_hand} in stock` : 'Out of stock' }}
              </p>
            </button>
          </div>

          <!-- No Products -->
          <div v-else class="flex flex-col items-center justify-center py-12">
            <p class="text-gray-500 text-lg mb-2">No products found</p>
            <p class="text-sm text-gray-400">Add products in Inventory first</p>
          </div>
        </div>
      </div>

      <!-- Right Sidebar - Shopping Cart -->
      <div class="w-96 bg-white border-l border-gray-200 flex flex-col shadow-lg">
        <!-- Cart Header -->
        <div class="bg-gradient-to-r from-primary-500 to-primary-600 text-white px-6 py-4 border-b">
          <h2 class="text-lg font-bold">🛒 Shopping Cart</h2>
          <p class="text-sm text-primary-100">{{ cartItems.length }} item(s)</p>
        </div>

        <!-- Cart Items -->
        <div class="flex-1 overflow-y-auto">
          <div v-if="cartItems.length > 0" class="divide-y">
            <div
              v-for="(item, idx) in cartItems"
              :key="idx"
              class="p-4 hover:bg-gray-50 transition-colors"
            >
              <!-- Item Header -->
              <div class="flex items-start justify-between mb-2">
                <div>
                  <h4 class="font-semibold text-gray-900 text-sm">{{ item.product_name }}</h4>
                  <p class="text-xs text-gray-500">₱{{ formatPrice(item.selling_price) }} each</p>
                </div>
                <button
                  @click="removeFromCart(idx)"
                  class="text-red-500 hover:text-red-700 text-sm font-bold"
                >
                  ✕
                </button>
              </div>

              <!-- Quantity Controls -->
              <div class="flex items-center gap-2 mb-2">
                <button
                  @click="decrementQuantity(idx)"
                  class="px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded text-sm font-bold"
                >
                  −
                </button>
                <input
                  :value="item.quantity"
                  @input="updateQuantity(idx, $event.target.value)"
                  type="number"
                  class="w-12 text-center px-2 py-1 border border-gray-300 rounded text-sm"
                />
                <button
                  @click="incrementQuantity(idx)"
                  class="px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded text-sm font-bold"
                >
                  +
                </button>
                <span class="text-xs text-gray-500 ml-auto">of {{ item.quantity_available }}</span>
              </div>

              <!-- Item Total -->
              <div class="text-right">
                <p class="text-sm font-bold text-gray-900">₱{{ formatPrice(item.quantity * item.selling_price) }}</p>
              </div>

              <!-- Discount for Item (Optional) -->
              <div class="mt-2 pt-2 border-t border-gray-200">
                <label class="text-xs text-gray-600">Discount: </label>
                <input
                  :value="item.discount || 0"
                  @input="updateDiscount(idx, $event.target.value)"
                  type="number"
                  min="0"
                  max="100"
                  class="w-16 px-2 py-1 border border-gray-300 rounded text-xs"
                />
                <span class="text-xs text-gray-500">%</span>
              </div>
            </div>
          </div>

          <!-- Empty Cart -->
          <div v-else class="flex flex-col items-center justify-center h-64">
            <p class="text-gray-400 text-base">🛍️</p>
            <p class="text-gray-500 text-sm mt-2">Cart is empty</p>
            <p class="text-xs text-gray-400">Select products to get started</p>
          </div>
        </div>

        <!-- Cart Summary & Payment -->
        <div class="border-t bg-gray-50 p-4 space-y-3">
          <!-- Subtotal -->
          <div class="flex justify-between text-sm">
            <span class="text-gray-700">Subtotal:</span>
            <span class="font-semibold">₱{{ formatPrice(subtotal) }}</span>
          </div>

          <!-- Discount -->
          <div class="flex justify-between items-center text-sm">
            <span class="text-gray-700">Discount:</span>
            <div class="flex items-center gap-2">
              <input
                v-model.number="discountPercent"
                type="number"
                min="0"
                max="100"
                class="w-16 px-2 py-1 border border-gray-300 rounded text-sm"
              />
              <span class="text-xs">% or ₱</span>
              <input
                v-model.number="discountAmount"
                type="number"
                class="w-20 px-2 py-1 border border-gray-300 rounded text-sm"
              />
            </div>
          </div>
          <p class="text-xs text-gray-500 text-right">-₱{{ formatPrice(totalDiscount) }}</p>

          <!-- VAT -->
          <div class="flex justify-between text-sm border-t pt-3">
            <span class="text-gray-700">VAT (12%):</span>
            <span class="font-semibold">₱{{ formatPrice(vatAmount) }}</span>
          </div>

          <!-- Total -->
          <div class="flex justify-between text-lg bg-primary-50 p-3 rounded-lg border-2 border-primary-200">
            <span class="font-bold text-gray-900">TOTAL:</span>
            <span class="font-bold text-primary-600">₱{{ formatPrice(totalAmount) }}</span>
          </div>

          <!-- Payment Method -->
          <div class="space-y-2">
            <label class="text-sm font-medium text-gray-700">Payment Method:</label>
            <select v-model="paymentMethod" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm">
              <option value="cash">💵 Cash</option>
              <option value="card">💳 Card</option>
              <option value="check">✓ Check</option>
              <option value="mixed">Mixed</option>
            </select>
          </div>

          <!-- Amount Paid -->
          <div class="space-y-1">
            <label class="text-sm font-medium text-gray-700">Amount Paid:</label>
            <input
              v-model.number="amountPaid"
              type="number"
              placeholder="0.00"
              class="w-full px-3 py-3 border border-gray-300 rounded-lg text-2xl font-bold focus:outline-none focus:ring-2 focus:ring-primary-500"
            />
          </div>

          <!-- Change -->
          <div class="bg-green-50 p-3 rounded-lg border-2 border-green-200">
            <p class="text-xs text-green-700 mb-1">Change:</p>
            <p class="text-2xl font-bold text-green-600">₱{{ formatPrice(Math.max(0, amountPaid - totalAmount)) }}</p>
          </div>

          <!-- Complete Sale Button -->
          <button
            v-if="cartItems.length > 0"
            @click="completeSale"
            :disabled="amountPaid < totalAmount"
            :class="[
              'w-full py-4 rounded-lg font-bold text-lg transition-all duration-200',
              amountPaid >= totalAmount
                ? 'bg-green-500 hover:bg-green-600 text-white cursor-pointer'
                : 'bg-gray-300 text-gray-500 cursor-not-allowed'
            ]"
          >
            ✓ Complete Sale
          </button>

          <!-- Print Receipt Button -->
          <button
            v-if="cartItems.length > 0"
            @click="printReceipt"
            class="w-full py-2 rounded-lg font-medium text-sm text-gray-700 bg-gray-200 hover:bg-gray-300 transition-colors"
          >
            🖨️ Print Preview
          </button>
        </div>
      </div>
    </div>

    <!-- Loading Overlay -->
    <div v-if="isLoading" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6">
        <p class="text-center text-gray-700">Processing sale...</p>
      </div>
    </div>

    <!-- Success Message -->
    <transition name="fade">
      <div
        v-if="successMessage"
        class="fixed bottom-6 right-6 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg"
      >
        {{ successMessage }}
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { productsApi, categoriesApi, salesApi } from '@/utils/api'

const authStore = useAuthStore()

// State
const products = ref([])
const categories = ref([])
const cartItems = ref([])
const selectedCategory = ref(null)
const searchQuery = ref('')
const selectedCustomer = ref('')
const showCustomer = ref(false)
const discountPercent = ref(0)
const discountAmount = ref(0)
const paymentMethod = ref('cash')
const amountPaid = ref(0)
const isLoading = ref(false)
const successMessage = ref('')

// Computed
const filteredProducts = computed(() => {
  let filtered = products.value

  if (selectedCategory.value) {
    filtered = filtered.filter(p => p.category_id === selectedCategory.value)
  }

  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    filtered = filtered.filter(p =>
      p.name.toLowerCase().includes(q) ||
      p.sku.toLowerCase().includes(q)
    )
  }

  return filtered
})

const subtotal = computed(() =>
  cartItems.value.reduce((sum, item) => sum + item.quantity * item.selling_price, 0)
)

const totalDiscount = computed(() => {
  const percentDiscount = subtotal.value * (discountPercent.value / 100)
  return percentDiscount + (discountAmount.value || 0)
})

const vatAmount = computed(() => (subtotal.value - totalDiscount.value) * 0.12)

const totalAmount = computed(() => subtotal.value - totalDiscount.value + vatAmount.value)

const userAvatar = computed(() => {
  const initial = (authStore.user?.username || 'U').charAt(0).toUpperCase()
  return `https://ui-avatars.com/api/?name=${initial}&background=3b82f6&color=fff&size=40`
})

// Methods
function addToCart(product) {
  const existing = cartItems.value.find(item => item.product_id === product.id)

  if (existing) {
    if (existing.quantity < product.quantity_on_hand) {
      existing.quantity += 1
    }
  } else {
    cartItems.value.push({
      product_id: product.id,
      product_name: product.name,
      selling_price: product.selling_price,
      quantity: 1,
      quantity_available: product.quantity_on_hand,
      discount: 0,
    })
  }
}

function removeFromCart(idx) {
  cartItems.value.splice(idx, 1)
}

function incrementQuantity(idx) {
  const item = cartItems.value[idx]
  if (item.quantity < item.quantity_available) {
    item.quantity += 1
  }
}

function decrementQuantity(idx) {
  const item = cartItems.value[idx]
  if (item.quantity > 1) {
    item.quantity -= 1
  }
}

function updateQuantity(idx, value) {
  const item = cartItems.value[idx]
  const qty = Math.max(1, Math.min(parseInt(value) || 1, item.quantity_available))
  item.quantity = qty
}

function updateDiscount(idx, value) {
  cartItems.value[idx].discount = Math.max(0, Math.min(100, parseFloat(value) || 0))
}

function clearCart() {
  if (confirm('Are you sure you want to clear the cart?')) {
    cartItems.value = []
    amountPaid.value = 0
    discountPercent.value = 0
    discountAmount.value = 0
  }
}

async function completeSale() {
  if (cartItems.value.length === 0) {
    alert('Cart is empty')
    return
  }

  if (amountPaid.value < totalAmount.value) {
    alert('Insufficient payment')
    return
  }

  isLoading.value = true

  try {
    const saleRequest = {
      customer_id: null,
      items: cartItems.value.map(item => ({
        product_id: item.product_id,
        quantity: item.quantity,
        unit_price: item.selling_price,
        discount_percent: item.discount || 0,
      })),
      discount_amount: discountAmount.value,
      discount_percent: discountPercent.value,
      payment_method: paymentMethod.value,
      amount_paid: amountPaid.value,
      notes: selectedCustomer.value || '',
    }

    await salesApi.createSale(authStore.token, saleRequest)

    successMessage.value = '✓ Sale completed successfully!'
    setTimeout(() => {
      successMessage.value = ''
    }, 3000)

    // Clear cart
    cartItems.value = []
    amountPaid.value = 0
    discountPercent.value = 0
    discountAmount.value = 0
    selectedCustomer.value = ''

    // Reload products to update stock
    await loadProducts()
  } catch (err) {
    alert(`Error: ${err.message}`)
  } finally {
    isLoading.value = false
  }
}

function printReceipt() {
  const receipt = `
    RECEIPT
    ═══════════════════════
    Date: ${new Date().toLocaleString()}
    User: ${authStore.user?.username}
    
    Items:
    ${cartItems.value.map(item => `${item.product_name} x${item.quantity} = ₱${formatPrice(item.quantity * item.selling_price)}`).join('\n')}
    
    Subtotal: ₱${formatPrice(subtotal.value)}
    Discount: -₱${formatPrice(totalDiscount.value)}
    VAT: ₱${formatPrice(vatAmount.value)}
    TOTAL: ₱${formatPrice(totalAmount.value)}
    
    Amount Paid: ₱${formatPrice(amountPaid.value)}
    Change: ₱${formatPrice(Math.max(0, amountPaid.value - totalAmount.value))}
    
    Thank you for your purchase!
  `
  console.log(receipt)
  alert('Receipt printed to console')
}

function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

// Load data
async function loadProducts() {
  try {
    const filter = null
    const data = await productsApi.getProducts(authStore.token, filter)
    products.value = data.products || data || []
  } catch (err) {
    console.error('Error loading products:', err)
  }
}

async function loadCategories() {
  try {
    const data = await categoriesApi.getCategories(authStore.token)
    categories.value = data.categories || data || []
  } catch (err) {
    console.error('Error loading categories:', err)
  }
}

onMounted(async () => {
  await loadProducts()
  await loadCategories()
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
