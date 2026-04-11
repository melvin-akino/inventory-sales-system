<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Profit & Loss Report</h2>
      <button class="btn-secondary no-print" @click="print">🖨️ Print</button>
    </div>

    <div class="card mb-4 no-print flex flex-wrap gap-3 items-end">
      <div><label class="label">From</label><input v-model="filter.date_from" type="date" class="input w-40" /></div>
      <div><label class="label">To</label><input v-model="filter.date_to" type="date" class="input w-40" /></div>
      <button class="btn-primary" @click="load" :disabled="loading">{{ loading ? 'Loading…' : 'Generate Report' }}</button>
    </div>

    <div v-if="report">
      <!-- P&L Summary -->
      <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-6">
        <div class="card border-l-4 border-green-500">
          <p class="text-xs text-gray-500">Total Revenue</p>
          <p class="text-2xl font-bold text-green-600 mt-1">{{ formatCurrency(report.total_revenue) }}</p>
        </div>
        <div class="card border-l-4 border-red-400">
          <p class="text-xs text-gray-500">Cost of Goods Sold</p>
          <p class="text-2xl font-bold text-red-600 mt-1">{{ formatCurrency(report.total_cost) }}</p>
        </div>
        <div class="card border-l-4 border-primary-500">
          <p class="text-xs text-gray-500">Gross Profit</p>
          <p class="text-2xl font-bold mt-1" :class="report.gross_profit >= 0 ? 'text-primary-700' : 'text-red-700'">{{ formatCurrency(report.gross_profit) }}</p>
          <p class="text-xs text-gray-400 mt-1">Margin: {{ report.gross_margin_percent.toFixed(1) }}%</p>
        </div>
        <div class="card">
          <p class="text-xs text-gray-500">VAT Collected</p>
          <p class="text-xl font-bold text-blue-600 mt-1">{{ formatCurrency(report.total_vat_collected) }}</p>
        </div>
        <div class="card">
          <p class="text-xs text-gray-500">Total Discounts Given</p>
          <p class="text-xl font-bold text-orange-500 mt-1">{{ formatCurrency(report.total_discount_given) }}</p>
        </div>
      </div>

      <!-- By Category -->
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-700 mb-4">Performance by Category</h3>
        <div class="table-container">
          <table class="data-table">
            <thead><tr><th>Category</th><th>Qty Sold</th><th>Revenue</th><th>COGS</th><th>Gross Profit</th><th>Margin</th></tr></thead>
            <tbody>
              <tr v-for="c in report.by_category" :key="c.category_name">
                <td class="font-medium">{{ c.category_name }}</td>
                <td>{{ c.quantity_sold }}</td>
                <td>{{ formatCurrency(c.revenue) }}</td>
                <td>{{ formatCurrency(c.cost) }}</td>
                <td class="font-semibold" :class="c.profit >= 0 ? 'text-green-600' : 'text-red-600'">{{ formatCurrency(c.profit) }}</td>
                <td>{{ c.revenue > 0 ? ((c.profit / c.revenue) * 100).toFixed(1) : '0.0' }}%</td>
              </tr>
            </tbody>
          </table>
        </div>
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
  try { report.value = await reportsApi.getProfitLossReport(auth.token, filter.value) }
  catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

function print() { window.print() }
</script>
