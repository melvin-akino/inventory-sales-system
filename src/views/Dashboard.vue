<template>
  <div>
    <!-- Stat Cards -->
    <div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-4 mb-6">
      <div class="stat-card col-span-1">
        <div class="stat-icon bg-green-100 text-2xl">💰</div>
        <div>
          <p class="text-xs text-gray-500 font-medium">Sales Today</p>
          <p class="text-lg font-bold text-gray-900">{{ formatCurrency(stats.total_sales_today) }}</p>
          <p class="text-xs text-gray-400">{{ stats.total_transactions_today }} transactions</p>
        </div>
      </div>
      <div class="stat-card col-span-1">
        <div class="stat-icon bg-blue-100 text-2xl">📅</div>
        <div>
          <p class="text-xs text-gray-500 font-medium">This Month</p>
          <p class="text-lg font-bold text-gray-900">{{ formatCurrency(stats.sales_this_month) }}</p>
        </div>
      </div>
      <div class="stat-card col-span-1">
        <div class="stat-icon bg-purple-100 text-2xl">📦</div>
        <div>
          <p class="text-xs text-gray-500 font-medium">Products</p>
          <p class="text-lg font-bold text-gray-900">{{ stats.total_products }}</p>
        </div>
      </div>
      <div class="stat-card col-span-1 cursor-pointer" @click="$router.push('/inventory/products')">
        <div class="stat-icon bg-yellow-100 text-2xl">⚠️</div>
        <div>
          <p class="text-xs text-gray-500 font-medium">Low Stock</p>
          <p class="text-lg font-bold" :class="stats.low_stock_count > 0 ? 'text-yellow-600' : 'text-gray-900'">
            {{ stats.low_stock_count }}
          </p>
        </div>
      </div>
      <div class="stat-card col-span-1">
        <div class="stat-icon bg-indigo-100 text-2xl">👥</div>
        <div>
          <p class="text-xs text-gray-500 font-medium">Customers</p>
          <p class="text-lg font-bold text-gray-900">{{ stats.total_customers }}</p>
        </div>
      </div>
      <div class="stat-card col-span-1 cursor-pointer" @click="$router.push('/sales/pos')" v-if="can(SALES)">
        <div class="stat-icon bg-green-100 text-2xl">🛒</div>
        <div>
          <p class="text-xs text-gray-500 font-medium">New Sale</p>
          <p class="text-sm font-semibold text-primary-600">Open POS →</p>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Top Products -->
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-700 mb-4">Top Products This Month</h3>
        <div v-if="!stats.top_products?.length" class="text-center py-8 text-gray-400 text-sm">No sales data yet</div>
        <div v-else class="space-y-3">
          <div v-for="(p, i) in stats.top_products" :key="i" class="flex items-center gap-3">
            <span class="w-6 h-6 rounded-full bg-primary-100 text-primary-700 text-xs font-bold flex items-center justify-center flex-shrink-0">{{ i + 1 }}</span>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium text-gray-800 truncate">{{ p.product_name }}</p>
              <p class="text-xs text-gray-400">{{ p.quantity_sold }} units sold</p>
            </div>
            <p class="text-sm font-semibold text-gray-700 flex-shrink-0">{{ formatCurrency(p.total_revenue) }}</p>
          </div>
        </div>
      </div>

      <!-- Recent Sales -->
      <div class="card lg:col-span-2">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-sm font-semibold text-gray-700">Recent Sales</h3>
          <router-link to="/sales/history" class="text-xs text-primary-600 hover:underline">View all →</router-link>
        </div>
        <div v-if="!stats.recent_sales?.length" class="text-center py-8 text-gray-400 text-sm">No recent sales</div>
        <div v-else class="space-y-2">
          <div
            v-for="s in stats.recent_sales"
            :key="s.sale_number"
            class="flex items-center gap-3 p-3 rounded-lg bg-gray-50 hover:bg-gray-100 transition-colors"
          >
            <div class="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center text-sm flex-shrink-0">🧾</div>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium text-gray-900">{{ s.sale_number }}</p>
              <p class="text-xs text-gray-400">{{ s.customer_name ?? 'Walk-in Customer' }} · {{ paymentLabel(s.payment_method) }}</p>
            </div>
            <div class="text-right flex-shrink-0">
              <p class="text-sm font-semibold text-gray-900">{{ formatCurrency(s.total_amount) }}</p>
              <p class="text-xs text-gray-400">{{ formatDateTime(s.sale_date) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { reportsApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency, formatDateTime, paymentLabel, ACCESS } from '@/utils/format'

const authStore = useAuthStore()
const appStore = useAppStore()
const { SALES } = ACCESS
const can = (roles) => authStore.can(roles)

const stats = ref({
  total_sales_today: 0,
  total_transactions_today: 0,
  total_products: 0,
  low_stock_count: 0,
  total_customers: 0,
  sales_this_month: 0,
  top_products: [],
  recent_sales: [],
})

onMounted(async () => {
  try {
    stats.value = await reportsApi.getDashboardStats(authStore.token)
  } catch (e) {
    appStore.notify(e.message, 'error')
  }
})
</script>
