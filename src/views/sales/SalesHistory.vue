<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Sales History</h2>
    </div>

    <!-- Filters -->
    <div class="card mb-4 flex flex-wrap gap-3">
      <input v-model="filter.search" class="input w-52" placeholder="🔍 Sale # or Customer" @input="load" />
      <input v-model="filter.date_from" type="date" class="input w-40" @change="load" />
      <input v-model="filter.date_to" type="date" class="input w-40" @change="load" />
      <select v-model="filter.payment_method" class="input w-40" @change="load">
        <option value="">All Payment</option>
        <option v-for="m in PAYMENT_METHODS" :key="m.value" :value="m.value">{{ m.label }}</option>
      </select>
      <select v-model="filter.status" class="input w-36" @change="load">
        <option value="">All Status</option>
        <option value="completed">Completed</option>
        <option value="void">Voided</option>
      </select>
    </div>

    <!-- Summary -->
    <div v-if="sales.length" class="grid grid-cols-3 gap-4 mb-4">
      <div class="card text-center">
        <p class="text-2xl font-bold text-gray-900">{{ completedSales.length }}</p>
        <p class="text-xs text-gray-500 mt-1">Completed Sales</p>
      </div>
      <div class="card text-center">
        <p class="text-2xl font-bold text-primary-700">{{ formatCurrency(totalRevenue) }}</p>
        <p class="text-xs text-gray-500 mt-1">Total Revenue</p>
      </div>
      <div class="card text-center">
        <p class="text-2xl font-bold text-gray-700">{{ formatCurrency(totalVat) }}</p>
        <p class="text-xs text-gray-500 mt-1">Total VAT Collected</p>
      </div>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>Sale #</th>
            <th>Date</th>
            <th>Customer</th>
            <th>Cashier</th>
            <th>Subtotal</th>
            <th>VAT</th>
            <th>Total</th>
            <th>Payment</th>
            <th>Status</th>
            <th v-if="can(MANAGE)">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="loading"><td colspan="10" class="text-center py-8 text-gray-400">Loading…</td></tr>
          <tr v-else-if="!sales.length"><td colspan="10" class="text-center py-8 text-gray-400">No sales found</td></tr>
          <tr v-for="s in sales" :key="s.id" class="cursor-pointer" @click="viewSale(s)">
            <td class="font-mono text-xs font-semibold">{{ s.sale_number }}</td>
            <td class="text-xs">{{ formatDateTime(s.sale_date) }}</td>
            <td>{{ s.customer_name ?? 'Walk-in' }}</td>
            <td class="text-gray-500">{{ s.cashier_name }}</td>
            <td>{{ formatCurrency(s.subtotal) }}</td>
            <td>{{ formatCurrency(s.vat_amount) }}</td>
            <td class="font-semibold">{{ formatCurrency(s.total_amount) }}</td>
            <td>
              <span class="badge bg-blue-50 text-blue-700">{{ paymentLabel(s.payment_method) }}</span>
            </td>
            <td>
              <span class="badge" :class="s.status === 'completed' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'">
                {{ s.status }}
              </span>
            </td>
            <td v-if="can(MANAGE)">
              <button v-if="s.status === 'completed'" class="btn btn-sm btn-danger" @click.stop="openVoid(s)">Void</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Sale Detail Modal -->
    <Modal v-model="showDetail" :title="selectedSale?.sale_number" width="640px">
      <div v-if="selectedSale" class="space-y-4">
        <div class="grid grid-cols-2 gap-4 text-sm">
          <div><span class="text-gray-500">Date:</span> {{ formatDateTime(selectedSale.sale_date) }}</div>
          <div><span class="text-gray-500">Cashier:</span> {{ selectedSale.cashier_name }}</div>
          <div><span class="text-gray-500">Customer:</span> {{ selectedSale.customer_name ?? 'Walk-in' }}</div>
          <div><span class="text-gray-500">Payment:</span> {{ paymentLabel(selectedSale.payment_method) }}</div>
        </div>
        <table class="data-table">
          <thead><tr><th>Product</th><th>Qty</th><th>Price</th><th>Disc%</th><th>Total</th></tr></thead>
          <tbody>
            <tr v-for="item in saleItems" :key="item.id">
              <td>{{ item.product_name }}</td>
              <td>{{ item.quantity }}</td>
              <td>{{ formatCurrency(item.unit_price) }}</td>
              <td>{{ item.discount_percent }}%</td>
              <td class="font-semibold">{{ formatCurrency(item.total_price) }}</td>
            </tr>
          </tbody>
        </table>
        <div class="bg-gray-50 rounded-lg p-4 space-y-1 text-sm">
          <div class="flex justify-between"><span class="text-gray-500">Subtotal</span><span>{{ formatCurrency(selectedSale.subtotal) }}</span></div>
          <div class="flex justify-between"><span class="text-gray-500">Discount</span><span>-{{ formatCurrency(selectedSale.discount_amount) }}</span></div>
          <div class="flex justify-between"><span class="text-gray-500">VAT (12%)</span><span>{{ formatCurrency(selectedSale.vat_amount) }}</span></div>
          <div class="flex justify-between font-bold text-base border-t pt-2"><span>Total</span><span>{{ formatCurrency(selectedSale.total_amount) }}</span></div>
          <div class="flex justify-between text-gray-500"><span>Amount Paid</span><span>{{ formatCurrency(selectedSale.amount_paid) }}</span></div>
          <div class="flex justify-between text-green-600 font-semibold"><span>Change</span><span>{{ formatCurrency(selectedSale.change_amount) }}</span></div>
        </div>
      </div>
    </Modal>

    <!-- Void Modal -->
    <Modal v-model="showVoidModal" title="Void Sale" width="420px">
      <p class="text-sm text-gray-600 mb-4">You are about to void <strong>{{ voidingSale?.sale_number }}</strong>. Stock will be restored.</p>
      <div class="space-y-3">
        <div>
          <label class="label">Reason for voiding *</label>
          <input v-model="voidReason" class="input" placeholder="e.g. Customer cancelled order" required />
        </div>
        <div v-if="voidError" class="text-sm text-red-600">{{ voidError }}</div>
      </div>
      <template #footer>
        <button class="btn-secondary" @click="showVoidModal = false">Cancel</button>
        <button class="btn-danger" :disabled="!voidReason || voiding" @click="confirmVoid">{{ voiding ? 'Voiding…' : 'Void Sale' }}</button>
      </template>
    </Modal>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { salesApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency, formatDateTime, paymentLabel, PAYMENT_METHODS, ACCESS, todayISO, firstDayOfMonth } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'

const auth = useAuthStore()
const appStore = useAppStore()
const { MANAGE } = ACCESS
const can = (r) => auth.can(r)

const sales = ref([])
const loading = ref(false)
const showDetail = ref(false)
const selectedSale = ref(null)
const saleItems = ref([])
const showVoidModal = ref(false)
const voidingSale = ref(null)
const voidReason = ref('')
const voidError = ref('')
const voiding = ref(false)

const filter = ref({ search: '', date_from: firstDayOfMonth(), date_to: todayISO(), payment_method: '', status: '' })

const completedSales = computed(() => sales.value.filter((s) => s.status === 'completed'))
const totalRevenue = computed(() => completedSales.value.reduce((s, x) => s + x.total_amount, 0))
const totalVat = computed(() => completedSales.value.reduce((s, x) => s + x.vat_amount, 0))

async function load() {
  loading.value = true
  try {
    sales.value = await salesApi.getSales(auth.token, {
      search: filter.value.search || null,
      date_from: filter.value.date_from || null,
      date_to: filter.value.date_to || null,
      payment_method: filter.value.payment_method || null,
      status: filter.value.status || null,
    })
  } catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

async function viewSale(s) {
  const full = await salesApi.getSale(auth.token, s.id)
  selectedSale.value = full
  saleItems.value = full.items
  showDetail.value = true
}

function openVoid(s) { voidingSale.value = s; voidReason.value = ''; voidError.value = ''; showVoidModal.value = true }

async function confirmVoid() {
  voiding.value = true; voidError.value = ''
  try {
    await salesApi.voidSale(auth.token, voidingSale.value.id, voidReason.value)
    appStore.notify('Sale voided and stock restored')
    showVoidModal.value = false
    load()
  } catch (e) { voidError.value = e.message }
  finally { voiding.value = false }
}

onMounted(load)
</script>
