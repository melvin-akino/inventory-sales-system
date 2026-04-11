<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Stock Adjustments</h2>
    </div>
    <div class="table-container">
      <table class="data-table">
        <thead><tr><th>Date</th><th>Product</th><th>Type</th><th>Before</th><th>Change</th><th>After</th><th>Reason</th><th>By</th></tr></thead>
        <tbody>
          <tr v-if="loading"><td colspan="8" class="text-center py-8 text-gray-400">Loading…</td></tr>
          <tr v-else-if="!adjustments.length"><td colspan="8" class="text-center py-8 text-gray-400">No adjustments found</td></tr>
          <tr v-for="a in adjustments" :key="a.id">
            <td class="text-xs text-gray-400">{{ formatDateTime(a.created_at) }}</td>
            <td class="font-medium">{{ a.product_name }}</td>
            <td>
              <span class="badge" :class="a.adjustment_type === 'add' ? 'bg-green-100 text-green-700' : a.adjustment_type === 'subtract' ? 'bg-red-100 text-red-700' : 'bg-blue-100 text-blue-700'">
                {{ a.adjustment_type }}
              </span>
            </td>
            <td>{{ a.quantity_before }}</td>
            <td class="font-semibold" :class="a.quantity_change >= 0 ? 'text-green-600' : 'text-red-600'">
              {{ a.quantity_change >= 0 ? '+' : '' }}{{ a.quantity_change }}
            </td>
            <td class="font-semibold">{{ a.quantity_after }}</td>
            <td class="text-gray-500">{{ a.reason ?? '—' }}</td>
            <td class="text-gray-500">{{ a.adjusted_by }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { productsApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatDateTime } from '@/utils/format'

const auth = useAuthStore()
const appStore = useAppStore()
const adjustments = ref([])
const loading = ref(false)

async function load() {
  loading.value = true
  try { adjustments.value = await productsApi.getStockAdjustments(auth.token) }
  catch (e) { appStore.notify(e.message, 'error') }
  finally { loading.value = false }
}

onMounted(load)
</script>
