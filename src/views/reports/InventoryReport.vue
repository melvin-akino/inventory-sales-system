<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Inventory Report</h2>
      <div class="flex gap-2 no-print">
        <button class="btn-primary" @click="load" :disabled="loading">{{ loading ? 'Loading…' : '↻ Refresh' }}</button>
        <button class="btn-secondary" @click="print">🖨️ Print</button>
      </div>
    </div>

    <div v-if="report">
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
        <div class="card text-center"><p class="text-xl font-bold text-gray-900">{{ report.total_items }}</p><p class="text-xs text-gray-500 mt-1">Active Products</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-primary-700">{{ formatCurrency(report.total_inventory_value) }}</p><p class="text-xs text-gray-500 mt-1">Inventory Value</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-yellow-600">{{ report.low_stock_count }}</p><p class="text-xs text-gray-500 mt-1">Low Stock Items</p></div>
        <div class="card text-center"><p class="text-xl font-bold text-red-600">{{ report.out_of_stock_count }}</p><p class="text-xs text-gray-500 mt-1">Out of Stock</p></div>
      </div>

      <div class="mb-4 no-print">
        <input v-model="search" class="input w-64" placeholder="🔍 Filter by product…" />
      </div>

      <div class="table-container">
        <table class="data-table">
          <thead><tr><th>SKU</th><th>Product</th><th>Category</th><th>Unit</th><th>In Stock</th><th>Reorder Lvl</th><th>Cost Price</th><th>Selling Price</th><th>Inv. Value</th><th>Status</th></tr></thead>
          <tbody>
            <tr v-for="item in filteredItems" :key="item.sku">
              <td class="font-mono text-xs">{{ item.sku }}</td>
              <td class="font-medium">{{ item.product_name }}</td>
              <td class="text-gray-500">{{ item.category ?? '—' }}</td>
              <td class="text-gray-500">{{ item.unit }}</td>
              <td class="font-bold" :class="item.quantity === 0 ? 'text-red-600' : item.quantity <= item.reorder_level ? 'text-yellow-600' : 'text-gray-900'">{{ item.quantity }}</td>
              <td class="text-gray-500">{{ item.reorder_level }}</td>
              <td>{{ formatCurrency(item.cost_price) }}</td>
              <td>{{ formatCurrency(item.selling_price) }}</td>
              <td class="font-semibold">{{ formatCurrency(item.inventory_value) }}</td>
              <td><span class="badge text-xs" :class="item.status === 'In Stock' ? 'bg-green-100 text-green-700' : item.status === 'Low Stock' ? 'bg-yellow-100 text-yellow-700' : 'bg-red-100 text-red-700'">{{ item.status }}</span></td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    <div v-else class="text-center py-16 text-gray-400">
      <button class="btn-primary" @click="load">Load Inventory Report</button>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { reportsApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency } from '@/utils/format'

const auth = useAuthStore()
const appStore = useAppStore()
const report = ref(null)
const loading = ref(false)
const search = ref('')

const filteredItems = computed(() => {
  if (!report.value) return []
  const s = search.value.toLowerCase()
  return s ? report.value.items.filter((i) => i.product_name.toLowerCase().includes(s) || i.sku.toLowerCase().includes(s)) : report.value.items
})

async function load() {
  loading.value = true
  try { report.value = await reportsApi.getInventoryReport(auth.token) }
  catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

function print() { window.print() }
onMounted(load)
</script>
