<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Invoices / Official Receipts</h2>
    </div>

    <div class="card mb-4 flex flex-wrap gap-3">
      <input v-model="filter.search" class="input w-56" placeholder="🔍 Invoice # or Customer" @input="load" />
      <input v-model="filter.date_from" type="date" class="input w-40" @change="load" />
      <input v-model="filter.date_to" type="date" class="input w-40" @change="load" />
      <select v-model="filter.status" class="input w-36" @change="load">
        <option value="">All Status</option>
        <option value="paid">Paid</option>
        <option value="void">Void</option>
      </select>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>OR/Invoice #</th>
            <th>Date</th>
            <th>Sale #</th>
            <th>Customer</th>
            <th>TIN</th>
            <th>VAT</th>
            <th>Total</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="loading"><td colspan="9" class="text-center py-8 text-gray-400">Loading…</td></tr>
          <tr v-else-if="!invoices.length"><td colspan="9" class="text-center py-8 text-gray-400">No invoices found</td></tr>
          <tr v-for="inv in invoices" :key="inv.id">
            <td class="font-mono text-xs font-semibold text-primary-700">{{ inv.invoice_number }}</td>
            <td class="text-xs">{{ formatDate(inv.invoice_date) }}</td>
            <td class="font-mono text-xs">{{ inv.sale_number }}</td>
            <td>{{ inv.customer_name ?? 'Walk-in Customer' }}</td>
            <td class="font-mono text-xs text-gray-500">{{ inv.customer_tin ?? '—' }}</td>
            <td>{{ formatCurrency(inv.vat_amount) }}</td>
            <td class="font-semibold">{{ formatCurrency(inv.total_amount) }}</td>
            <td>
              <span class="badge" :class="inv.status === 'paid' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'">
                {{ inv.status }}
              </span>
            </td>
            <td>
              <button class="btn btn-sm btn-secondary" @click="viewInvoice(inv)">View / Print</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Invoice Print Modal -->
    <Modal v-model="showInvoice" :title="`Official Receipt — ${selectedInvoice?.invoice_number}`" width="640px">
      <div v-if="selectedInvoice" id="invoice-print" class="space-y-4 font-sans text-sm">
        <!-- Header -->
        <div class="text-center border-b pb-4">
          <h2 class="text-xl font-bold">{{ appStore.getSetting('company_name') }}</h2>
          <p class="text-gray-500">{{ appStore.getSetting('company_address') }}</p>
          <p class="text-gray-500">TIN: {{ appStore.getSetting('company_tin', 'N/A') }}</p>
          <p class="text-gray-500">Tel: {{ appStore.getSetting('company_phone', 'N/A') }}</p>
        </div>

        <div class="text-center">
          <p class="text-lg font-bold">OFFICIAL RECEIPT</p>
          <p class="font-mono text-sm">{{ selectedInvoice.invoice_number }}</p>
          <p class="text-gray-500 text-xs">Date: {{ formatDate(selectedInvoice.invoice_date) }}</p>
        </div>

        <div class="grid grid-cols-2 gap-2 text-xs border p-3 rounded-lg">
          <div><span class="text-gray-500">Customer:</span> {{ selectedInvoice.customer_name ?? 'Walk-in Customer' }}</div>
          <div><span class="text-gray-500">TIN:</span> {{ selectedInvoice.customer_tin ?? 'N/A' }}</div>
          <div class="col-span-2"><span class="text-gray-500">Address:</span> {{ selectedInvoice.customer_address ?? 'N/A' }}</div>
        </div>

        <table class="w-full text-xs border-collapse">
          <thead>
            <tr class="border-b-2 border-gray-300">
              <th class="text-left py-2">Item</th>
              <th class="text-right py-2">Qty</th>
              <th class="text-right py-2">Unit Price</th>
              <th class="text-right py-2">Amount</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in saleDetail?.items" :key="item.id" class="border-b border-gray-100">
              <td class="py-1">{{ item.product_name }}</td>
              <td class="text-right">{{ item.quantity }}</td>
              <td class="text-right">{{ formatCurrency(item.unit_price) }}</td>
              <td class="text-right font-medium">{{ formatCurrency(item.total_price) }}</td>
            </tr>
          </tbody>
        </table>

        <div class="border-t pt-3 space-y-1 text-xs">
          <div class="flex justify-between"><span class="text-gray-500">VATable Sales</span><span>{{ formatCurrency(saleDetail?.subtotal) }}</span></div>
          <div class="flex justify-between"><span class="text-gray-500">VAT Amount (12%)</span><span>{{ formatCurrency(selectedInvoice.vat_amount) }}</span></div>
          <div class="flex justify-between font-bold text-sm border-t pt-2"><span>TOTAL AMOUNT DUE</span><span>{{ formatCurrency(selectedInvoice.total_amount) }}</span></div>
        </div>

        <p class="text-center text-xs text-gray-400 border-t pt-3">{{ appStore.getSetting('receipt_footer', 'Thank you for your business!') }}</p>
        <p class="text-center text-xs text-gray-300">This serves as your Official Receipt</p>
      </div>
      <template #footer>
        <button class="btn-secondary" @click="showInvoice = false">Close</button>
        <button class="btn-primary" @click="printInvoice">🖨️ Print</button>
      </template>
    </Modal>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoicesApi, salesApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency, formatDate, todayISO, firstDayOfMonth } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'

const auth = useAuthStore()
const appStore = useAppStore()
const invoices = ref([])
const loading = ref(false)
const showInvoice = ref(false)
const selectedInvoice = ref(null)
const saleDetail = ref(null)
const filter = ref({ search: '', date_from: firstDayOfMonth(), date_to: todayISO(), status: '' })

async function load() {
  loading.value = true
  try {
    invoices.value = await invoicesApi.getInvoices(auth.token, {
      search: filter.value.search || null,
      date_from: filter.value.date_from || null,
      date_to: filter.value.date_to || null,
      status: filter.value.status || null,
    })
  } catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

async function viewInvoice(inv) {
  selectedInvoice.value = inv
  try { saleDetail.value = await salesApi.getSale(auth.token, inv.sale_id) } catch (_) {}
  showInvoice.value = true
}

function printInvoice() { window.print() }

onMounted(load)
</script>
