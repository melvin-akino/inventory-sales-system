<template>
  <div class="flex gap-4 h-full" style="height: calc(100vh - 120px)">
    <!-- Product Search Panel -->
    <div class="flex-1 flex flex-col gap-4 overflow-hidden">
      <div class="card flex-shrink-0">
        <div class="flex gap-3">
          <input v-model="search" class="input flex-1" placeholder="🔍 Search product name or SKU…" @input="searchProducts" />
          <select v-model="selectedCategory" class="input w-44" @change="searchProducts">
            <option :value="null">All Categories</option>
            <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto">
        <div class="grid grid-cols-2 xl:grid-cols-3 gap-3">
          <button
            v-for="p in products"
            :key="p.id"
            @click="addToCart(p)"
            class="card text-left hover:border-primary-300 hover:shadow-md transition-all cursor-pointer border-2"
            :class="p.quantity === 0 ? 'opacity-50 cursor-not-allowed' : 'border-transparent'"
            :disabled="p.quantity === 0"
          >
            <p class="font-medium text-sm text-gray-900 leading-tight">{{ p.name }}</p>
            <p class="text-xs text-gray-400 mt-0.5">{{ p.sku }} · {{ p.category_name }}</p>
            <div class="flex items-center justify-between mt-2">
              <span class="text-sm font-bold text-primary-700">{{ formatCurrency(p.selling_price) }}</span>
              <span class="text-xs badge" :class="stockStatusColor(p.quantity, p.reorder_level)">{{ p.quantity }} left</span>
            </div>
          </button>
          <div v-if="!products.length" class="col-span-3 text-center py-12 text-gray-400">No products found</div>
        </div>
      </div>
    </div>

    <!-- Cart Panel - IMPROVED -->
    <div class="w-full lg:w-[500px] flex flex-col gap-3 flex-shrink-0 bg-white rounded-lg shadow-lg">
      <!-- Header -->
      <div class="px-6 pt-6 pb-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold text-gray-900">Shopping Cart</h2>
          <button v-if="cart.length" @click="cart = []" class="text-sm text-red-500 hover:text-red-700 font-medium">
            Clear All
          </button>
        </div>
        <p class="text-sm text-gray-500 mt-1">{{ cart.length }} item(s) in cart</p>
      </div>

      <!-- Customer Selection -->
      <div class="px-6">
        <label class="label text-sm font-semibold text-gray-700">Customer (Optional)</label>
        <select v-model="selectedCustomer" class="input text-base">
          <option :value="null">Walk-in Customer</option>
          <option v-for="c in customers" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
      </div>

      <!-- Cart Items - LARGER AND MORE VISIBLE -->
      <div class="flex-1 overflow-y-auto px-6">
        <div v-if="!cart.length" class="flex flex-col items-center justify-center h-full text-gray-400">
          <svg class="w-16 h-16 mb-3 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2 8m10 0l2-8m0 0h-4m4 0l2 8M9 21v-2M15 21v-2" />
          </svg>
          <p class="text-lg font-medium">Cart is empty</p>
          <p class="text-sm">Add products to get started</p>
        </div>

        <div class="space-y-3">
          <div
            v-for="(item, i) in cart"
            :key="i"
            class="bg-gradient-to-r from-gray-50 to-white rounded-lg border border-gray-200 p-4 hover:shadow-md transition-shadow"
          >
            <!-- Product Info -->
            <div class="flex justify-between items-start mb-3">
              <div class="flex-1">
                <p class="font-semibold text-gray-900 text-base">{{ item.name }}</p>
                <p class="text-sm text-gray-500 mt-0.5">{{ formatCurrency(item.unit_price) }} each</p>
              </div>
              <button
                @click="removeItem(i)"
                class="text-red-500 hover:text-red-700 hover:bg-red-50 rounded-full p-1 transition-colors"
                title="Remove item"
              >
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                  <path
                    fill-rule="evenodd"
                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
            </div>

            <!-- Quantity Controls - LARGER -->
            <div class="flex items-center gap-3 mb-3">
              <button
                @click="decreaseQty(i)"
                class="w-10 h-10 rounded-lg bg-gray-200 text-gray-700 font-bold text-lg hover:bg-gray-300 transition-colors"
              >
                −
              </button>
              <input
                v-model.number="item.quantity"
                type="number"
                min="1"
                :max="item.max_qty"
                @change="validateQty(i)"
                class="w-16 text-center text-lg font-bold border-2 border-gray-300 rounded-lg py-2"
              />
              <button
                @click="increaseQty(i)"
                class="w-10 h-10 rounded-lg bg-primary-500 text-white font-bold text-lg hover:bg-primary-600 transition-colors"
              >
                +
              </button>
              <span class="text-sm text-gray-500 ml-2">of {{ item.max_qty }} available</span>
            </div>

            <!-- Discount -->
            <div class="mb-3">
              <label class="text-sm text-gray-600 font-medium">Discount</label>
              <div class="flex gap-2 mt-1">
                <input
                  v-model.number="item.discount_percent"
                  type="number"
                  min="0"
                  max="100"
                  step="0.5"
                  class="flex-1 text-sm border-2 border-gray-300 rounded-lg px-3 py-2 font-medium"
                  placeholder="0"
                />
                <span class="text-sm font-bold text-gray-700 py-2">%</span>
              </div>
            </div>

            <!-- Item Total -->
            <div class="flex justify-between items-center bg-white rounded-lg p-3 border-2 border-primary-200">
              <span class="text-sm font-semibold text-gray-600">Subtotal</span>
              <span class="text-lg font-bold text-primary-700">{{ formatCurrency(itemTotal(item)) }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Divider -->
      <div class="border-t border-gray-200"></div>

      <!-- Totals Summary -->
      <div class="px-6 py-4 space-y-3 bg-gray-50 rounded-b-lg">
        <div class="space-y-2">
          <div class="flex justify-between text-base">
            <span class="text-gray-600">Subtotal</span>
            <span class="font-semibold text-gray-900">{{ formatCurrency(subtotal) }}</span>
          </div>

          <div class="flex justify-between text-base items-center">
            <span class="text-gray-600">Discount</span>
            <div class="flex items-center gap-2">
              <span class="text-sm text-gray-500">₱</span>
              <input
                v-model.number="extraDiscount"
                type="number"
                min="0"
                step="0.01"
                class="w-32 text-right text-base font-semibold border-2 border-gray-300 rounded-lg px-3 py-2"
              />
            </div>
          </div>

          <div class="flex justify-between text-base">
            <span class="text-gray-600">VAT ({{ vatRate }}%)</span>
            <span class="font-semibold text-gray-900">{{ formatCurrency(vatAmount) }}</span>
          </div>
        </div>

        <!-- Grand Total -->
        <div class="bg-white rounded-lg p-4 border-2 border-primary-300">
          <div class="flex justify-between items-center">
            <span class="text-lg font-bold text-gray-900">TOTAL</span>
            <span class="text-3xl font-bold text-primary-700">{{ formatCurrency(grandTotal) }}</span>
          </div>
        </div>

        <!-- Payment Method -->
        <div>
          <label class="label text-sm font-semibold text-gray-700">Payment Method</label>
          <select v-model="paymentMethod" class="input text-base">
            <option v-for="m in PAYMENT_METHODS" :key="m.value" :value="m.value">{{ m.label }}</option>
          </select>
        </div>

        <!-- Amount Paid -->
        <div>
          <label class="label text-sm font-semibold text-gray-700">Amount Paid (₱)</label>
          <input
            v-model.number="amountPaid"
            type="number"
            min="0"
            step="0.01"
            class="input text-2xl font-bold text-primary-700 text-center"
          />
        </div>

        <!-- Change Display -->
        <div v-if="amountPaid >= grandTotal" class="bg-green-50 rounded-lg p-4 border-2 border-green-300">
          <div class="flex justify-between items-center">
            <span class="text-lg font-semibold text-green-700">Change</span>
            <span class="text-2xl font-bold text-green-700">{{ formatCurrency(change) }}</span>
          </div>
        </div>

        <div v-if="saleError" class="text-sm text-red-600 bg-red-50 rounded-lg p-3 font-medium">
          {{ saleError }}
        </div>

        <!-- Complete Sale Button -->
        <button
          class="btn-success w-full py-4 text-lg font-bold transition-all"
          :disabled="!cart.length || amountPaid < grandTotal || processing"
          @click="processSale"
        >
          {{ processing ? 'Processing…' : `✓ Complete Sale` }}
        </button>
      </div>
    </div>

    <!-- Receipt Modal -->
    <Modal v-model="showReceipt" title="Sale Complete" width="500px">
      <div class="text-center mb-6">
        <div class="text-6xl mb-3">✅</div>
        <p class="text-2xl font-bold text-gray-900">Sale Completed!</p>
        <p class="text-base text-gray-500 mt-2">{{ lastSale?.sale_number }}</p>
      </div>
      <div class="bg-gray-50 rounded-lg p-6 space-y-3 text-base">
        <div class="flex justify-between">
          <span class="text-gray-600">Total</span>
          <span class="font-bold text-lg">{{ formatCurrency(lastSale?.total_amount) }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Amount Paid</span>
          <span class="font-semibold">{{ formatCurrency(lastSale?.amount_paid) }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Change</span>
          <span class="font-bold text-green-600 text-lg">{{ formatCurrency(lastSale?.change_amount) }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Payment Method</span>
          <span class="font-semibold">{{ paymentLabel(lastSale?.payment_method) }}</span>
        </div>
        <div class="flex justify-between border-t pt-3">
          <span class="text-gray-600">OR/Invoice</span>
          <span class="font-mono text-sm font-semibold">Auto-generated</span>
        </div>
      </div>
      <template #footer>
        <button class="btn-secondary" @click="showReceipt = false; newSale()">New Sale</button>
        <button class="btn-primary" @click="printReceipt">🖨️ Print Receipt</button>
      </template>
    </Modal>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { productsApi, categoriesApi, salesApi, customersApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency, paymentLabel, stockStatusColor, stockStatusLabel, PAYMENT_METHODS } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'

const auth = useAuthStore()
const appStore = useAppStore()

const products = ref([])
const categories = ref([])
const customers = ref([])
const search = ref('')
const selectedCategory = ref(null)
const selectedCustomer = ref(null)
const cart = ref([])
const extraDiscount = ref(0)
const paymentMethod = ref('cash')
const amountPaid = ref(0)
const processing = ref(false)
const saleError = ref('')
const showReceipt = ref(false)
const lastSale = ref(null)
const vatRate = ref(12)

function itemTotal(item) {
  const base = item.unit_price * item.quantity
  const disc = base * (item.discount_percent / 100)
  return base - disc
}

const subtotal = computed(() => cart.value.reduce((s, i) => s + itemTotal(i), 0))
const vatAmount = computed(() => subtotal.value * (vatRate.value / 100))
const grandTotal = computed(() => subtotal.value + vatAmount.value - (extraDiscount.value || 0))
const change = computed(() => (amountPaid.value || 0) - grandTotal.value)

function addToCart(p) {
  const existing = cart.value.find((i) => i.product_id === p.id)
  if (existing) {
    if (existing.quantity < p.quantity) existing.quantity++
    else appStore.notify(`Only ${p.quantity} in stock`, 'warning')
  } else {
    cart.value.push({
      product_id: p.id,
      name: p.name,
      unit_price: p.selling_price,
      quantity: 1,
      discount_percent: 0,
      max_qty: p.quantity,
    })
  }
  if (!amountPaid.value || amountPaid.value < grandTotal.value) {
    amountPaid.value = Math.ceil(grandTotal.value)
  }
}

function increaseQty(i) {
  const item = cart.value[i]
  if (item.quantity < item.max_qty) item.quantity++
  else appStore.notify('Insufficient stock', 'warning')
}

function decreaseQty(i) {
  if (cart.value[i].quantity > 1) cart.value[i].quantity--
  else removeItem(i)
}

function validateQty(i) {
  const item = cart.value[i]
  if (item.quantity < 1) item.quantity = 1
  if (item.quantity > item.max_qty) {
    item.quantity = item.max_qty
    appStore.notify(`Maximum ${item.max_qty} available`, 'warning')
  }
}

function removeItem(i) {
  cart.value.splice(i, 1)
}

async function searchProducts() {
  try {
    products.value = await productsApi.getProducts(auth.token, {
      search: search.value || null,
      category_id: selectedCategory.value,
      active_only: true,
    })
  } catch (_) {}
}

async function processSale() {
  saleError.value = ''
  processing.value = true
  try {
    const sale = await salesApi.createSale(auth.token, {
      customer_id: selectedCustomer.value,
      items: cart.value.map((i) => ({
        product_id: i.product_id,
        quantity: i.quantity,
        unit_price: i.unit_price,
        discount_percent: i.discount_percent,
      })),
      discount_amount: extraDiscount.value || 0,
      amount_paid: amountPaid.value,
      payment_method: paymentMethod.value,
      notes: null,
    })
    lastSale.value = sale
    showReceipt.value = true
    searchProducts()
  } catch (e) {
    saleError.value = e.message
  } finally {
    processing.value = false
  }
}

function newSale() {
  cart.value = []
  extraDiscount.value = 0
  amountPaid.value = 0
  selectedCustomer.value = null
  saleError.value = ''
}

function printReceipt() {
  window.print()
}

onMounted(async () => {
  const settings = await appStore.loadSettings()
  vatRate.value = parseFloat(appStore.getSetting('vat_rate', '12'))
  await Promise.all([
    searchProducts(),
    categoriesApi.getCategories(auth.token).then((c) => (categories.value = c)),
    customersApi.getCustomers(auth.token).then((c) => (customers.value = c)),
  ])
})
</script>
