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

    <!-- Cart Panel -->
    <div class="w-96 flex flex-col gap-3 flex-shrink-0">
      <!-- Customer -->
      <div class="card flex-shrink-0">
        <label class="label">Customer (optional)</label>
        <div class="flex gap-2">
          <select v-model="selectedCustomer" class="input flex-1">
            <option :value="null">Walk-in Customer</option>
            <option v-for="c in customers" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>
      </div>

      <!-- Cart Items -->
      <div class="card flex-1 overflow-y-auto">
        <h3 class="text-sm font-semibold text-gray-700 mb-3 flex items-center justify-between">
          Cart
          <button v-if="cart.length" @click="cart = []" class="text-xs text-red-500 hover:text-red-700">Clear</button>
        </h3>
        <div v-if="!cart.length" class="text-center py-8 text-gray-400 text-sm">Cart is empty</div>
        <div class="space-y-2">
          <div v-for="(item, i) in cart" :key="i" class="flex items-center gap-2 p-2 rounded-lg bg-gray-50">
            <div class="flex-1 min-w-0">
              <p class="text-xs font-medium text-gray-900 truncate">{{ item.name }}</p>
              <p class="text-xs text-gray-400">{{ formatCurrency(item.unit_price) }} × {{ item.quantity }}</p>
              <div class="flex items-center gap-1 mt-1">
                <span class="text-xs text-gray-500">Disc:</span>
                <input v-model.number="item.discount_percent" type="number" min="0" max="100" step="0.5"
                  class="w-14 text-xs border border-gray-200 rounded px-1 py-0.5" />
                <span class="text-xs text-gray-500">%</span>
              </div>
            </div>
            <div class="flex flex-col items-end gap-1">
              <span class="text-sm font-semibold text-primary-700">{{ formatCurrency(itemTotal(item)) }}</span>
              <div class="flex items-center gap-1">
                <button @click="decreaseQty(i)" class="w-6 h-6 rounded bg-gray-200 text-gray-700 text-xs font-bold hover:bg-gray-300">−</button>
                <span class="w-6 text-center text-xs font-semibold">{{ item.quantity }}</span>
                <button @click="increaseQty(i)" class="w-6 h-6 rounded bg-gray-200 text-gray-700 text-xs font-bold hover:bg-gray-300">+</button>
                <button @click="removeItem(i)" class="w-6 h-6 rounded bg-red-100 text-red-600 text-xs hover:bg-red-200">✕</button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Totals & Payment -->
      <div class="card flex-shrink-0 space-y-3">
        <div class="flex justify-between text-sm">
          <span class="text-gray-500">Subtotal</span>
          <span class="font-medium">{{ formatCurrency(subtotal) }}</span>
        </div>
        <div class="flex justify-between text-sm items-center">
          <span class="text-gray-500">Discount (₱)</span>
          <input v-model.number="extraDiscount" type="number" min="0" step="0.01" class="w-28 text-sm text-right border border-gray-200 rounded px-2 py-1" />
        </div>
        <div class="flex justify-between text-sm">
          <span class="text-gray-500">VAT ({{ vatRate }}%)</span>
          <span class="font-medium">{{ formatCurrency(vatAmount) }}</span>
        </div>
        <div class="flex justify-between text-base font-bold border-t pt-2">
          <span>TOTAL</span>
          <span class="text-primary-700">{{ formatCurrency(grandTotal) }}</span>
        </div>

        <div>
          <label class="label">Payment Method</label>
          <select v-model="paymentMethod" class="input">
            <option v-for="m in PAYMENT_METHODS" :key="m.value" :value="m.value">{{ m.label }}</option>
          </select>
        </div>

        <div>
          <label class="label">Amount Paid (₱)</label>
          <input v-model.number="amountPaid" type="number" min="0" step="0.01" class="input text-lg font-bold" />
        </div>

        <div v-if="amountPaid >= grandTotal" class="flex justify-between text-sm bg-green-50 rounded-lg px-3 py-2">
          <span class="text-green-700 font-medium">Change</span>
          <span class="font-bold text-green-700">{{ formatCurrency(change) }}</span>
        </div>

        <div v-if="saleError" class="text-xs text-red-600">{{ saleError }}</div>

        <button
          class="btn-success w-full py-3 text-base font-bold"
          :disabled="!cart.length || amountPaid < grandTotal || processing"
          @click="processSale"
        >
          {{ processing ? 'Processing…' : '✓ Complete Sale' }}
        </button>
      </div>
    </div>

    <!-- Receipt Modal -->
    <Modal v-model="showReceipt" title="Sale Complete" width="420px">
      <div class="text-center mb-4">
        <div class="text-4xl mb-2">✅</div>
        <p class="text-lg font-bold text-gray-900">Sale Completed!</p>
        <p class="text-sm text-gray-500">{{ lastSale?.sale_number }}</p>
      </div>
      <div class="bg-gray-50 rounded-lg p-4 space-y-2 text-sm">
        <div class="flex justify-between"><span class="text-gray-500">Total</span><span class="font-bold">{{ formatCurrency(lastSale?.total_amount) }}</span></div>
        <div class="flex justify-between"><span class="text-gray-500">Amount Paid</span><span>{{ formatCurrency(lastSale?.amount_paid) }}</span></div>
        <div class="flex justify-between"><span class="text-gray-500">Change</span><span class="font-bold text-green-600">{{ formatCurrency(lastSale?.change_amount) }}</span></div>
        <div class="flex justify-between"><span class="text-gray-500">Payment</span><span>{{ paymentLabel(lastSale?.payment_method) }}</span></div>
        <div class="flex justify-between border-t pt-2"><span class="text-gray-500">OR/Invoice</span><span class="font-mono text-xs">Auto-generated</span></div>
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
    cart.value.push({ product_id: p.id, name: p.name, unit_price: p.selling_price, quantity: 1, discount_percent: 0, max_qty: p.quantity })
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

function removeItem(i) { cart.value.splice(i, 1) }

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
  await Promise.all([searchProducts(), categoriesApi.getCategories(auth.token).then(c => categories.value = c), customersApi.getCustomers(auth.token).then(c => customers.value = c)])
})
</script>
