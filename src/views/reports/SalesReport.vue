<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Sales Report</h2>
      <button class="btn-secondary no-print" @click="print">🖨️ Print</button>
    </div>

    <div class="card mb-4 no-print flex flex-wrap gap-3 items-end">
      <div><label class="label">From</label><input v-model="filter.date_from" type="date" class="input w-40" /></div>
      <div><label class="label">To</label><input v-model="filter.date_to" type="date" class="input w-40" /></div>
      <button class="btn-primary" @click="load" :disabled="loading">{{ loading ? 'Loading…' : 'Generate Report' }}</button>
    </div>

    <div v-if="report">
      <!-- Summary Cards -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4 mb-6">
        <div class="card text-center"><p class="text-xl font-bold text-gray-900">{{ report.transaction_count }}</p><p class="text-xs text-gray-500 mt-1">Transactions</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-gray-900">{{ formatCurrency(report.total_sales) }}</p><p class="text-xs text-gray-500 mt-1">Subtotal</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-red-600">-{{ formatCurrency(report.total_discount) }}</p><p class="text-xs text-gray-500 mt-1">Total Discount</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-blue-600">{{ formatCurrency(report.total_vat) }}</p><p class="text-xs text-gray-500 mt-1">VAT Collected</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-primary-700">{{ formatCurrency(report.grand_total) }}</p><p class="text-xs text-gray-500 mt-1">Grand Total</p></div>
      </div>

      <div class="table-container">
        <table class="data-table">
          <thead><tr><th>Date</th><th>Sale #</th><th>Customer</th><th>Cashier</th><th>Subtotal</th><th>Discount</th><th>VAT</th><th>Total</th><th>Payment</th><th>Status</th></tr></thead>
          <tbody>
            <tr v-for="item in report.items" :key="item.sale_number" :class="item.status === 'void' ? 'opacity-40 line-through' : ''">
              <td class="text-xs">{{ item.date }}</td>
              <td class="font-mono text-xs">{{ item.sale_number }}</td>
              <td>{{ item.customer_name ?? 'Walk-in' }}</td>
              <td class="text-gray-500">{{ item.cashier_name }}</td>
              <td>{{ formatCurrency(item.subtotal) }}</td>
              <td class="text-red-600">{{ item.discount > 0 ? `-${formatCurrency(item.discount)}` : '—' }}</td>
              <td>{{ formatCurrency(item.vat) }}</td>
              <td class="font-semibold">{{ formatCurrency(item.total) }}</td>
              <td><span class="badge bg-blue-50 text-blue-700 text-xs">{{ item.payment_method }}</span></td>
              <td><span class="badge text-xs" :class="item.status === 'completed' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'">{{ item.status }}</span></td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    <div v-else class="text-center py-16 text-gray-400">Select a date range and click Generate Report</div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { reportsApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency, firstDayOfMonth, todayISO } from '@/utils/format'

const auth = useAuthStore()
const appStore = useAppStore()
const report = ref(null)
const loading = ref(false)
const filter = ref({ date_from: firstDayOfMonth(), date_to: todayISO() })

async function load() {
  loading.value = true
  try { report.value = await reportsApi.getSalesReport(auth.token, filter.value) }
  catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

function print() { window.print() }
</script>
