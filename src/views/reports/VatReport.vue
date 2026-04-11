<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">VAT Report (BIR)</h2>
      <button class="btn-secondary no-print" @click="print">🖨️ Print</button>
    </div>

    <div class="card mb-4 no-print flex flex-wrap gap-3 items-end">
      <div><label class="label">From</label><input v-model="filter.date_from" type="date" class="input w-40" /></div>
      <div><label class="label">To</label><input v-model="filter.date_to" type="date" class="input w-40" /></div>
      <button class="btn-primary" @click="load" :disabled="loading">{{ loading ? 'Loading…' : 'Generate Report' }}</button>
    </div>

    <div v-if="report">
      <!-- BIR-style summary -->
      <div class="card mb-6">
        <h3 class="text-sm font-semibold mb-4 text-center">SUMMARY OF SALES — VAT Output</h3>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">VATable Sales</p>
            <p class="text-xl font-bold text-gray-900 mt-1">{{ formatCurrency(report.vatable_sales) }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">VAT Exempt Sales</p>
            <p class="text-xl font-bold text-gray-900 mt-1">{{ formatCurrency(report.vat_exempt_sales) }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Total Sales</p>
            <p class="text-xl font-bold text-gray-900 mt-1">{{ formatCurrency(report.total_sales) }}</p>
          </div>
          <div class="bg-blue-50 rounded-xl p-3">
            <p class="text-xs text-blue-600 uppercase tracking-wide font-semibold">Output VAT (12%)</p>
            <p class="text-xl font-bold text-blue-700 mt-1">{{ formatCurrency(report.output_vat) }}</p>
          </div>
        </div>
      </div>

      <div class="table-container">
        <table class="data-table">
          <thead>
            <tr>
              <th>OR Number</th>
              <th>Date</th>
              <th>Customer</th>
              <th>TIN</th>
              <th>VATable Amount</th>
              <th>VAT (12%)</th>
              <th>Total Amount</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in report.items" :key="item.invoice_number">
              <td class="font-mono text-xs">{{ item.invoice_number }}</td>
              <td class="text-xs">{{ item.date }}</td>
              <td>{{ item.customer_name ?? 'Walk-in Customer' }}</td>
              <td class="font-mono text-xs text-gray-500">{{ item.customer_tin ?? '—' }}</td>
              <td>{{ formatCurrency(item.vatable_amount) }}</td>
              <td class="text-blue-600 font-semibold">{{ formatCurrency(item.vat_amount) }}</td>
              <td class="font-semibold">{{ formatCurrency(item.total_amount) }}</td>
            </tr>
            <tr class="font-bold bg-gray-50">
              <td colspan="4" class="text-right">TOTAL</td>
              <td>{{ formatCurrency(report.vatable_sales) }}</td>
              <td class="text-blue-700">{{ formatCurrency(report.output_vat) }}</td>
              <td>{{ formatCurrency(report.total_sales) }}</td>
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
  try { report.value = await reportsApi.getVatReport(auth.token, filter.value) }
  catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

function print() { window.print() }
</script>
