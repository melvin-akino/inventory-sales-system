<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <h1 class="text-2xl font-bold text-gray-900">Admin - Orders Management</h1>
          <router-link to="/dashboard" class="text-blue-600 hover:text-blue-700">← Back to Dashboard</router-link>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- Loading State -->
      <div v-if="isLoading" class="text-center py-12">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
        <p class="mt-2 text-gray-600">Loading orders...</p>
      </div>

      <!-- Error State -->
      <div v-if="error" class="bg-red-50 border border-red-200 rounded-lg p-4 mb-4 text-red-700">
        {{ error }}
      </div>

      <!-- Orders Table -->
      <div v-if="orders.length > 0" class="bg-white rounded-lg border border-gray-200 overflow-hidden">
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead class="bg-gray-50 border-b border-gray-200">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Order #</th>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Customer</th>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Amount</th>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Payment</th>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Order Status</th>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Date</th>
                <th class="px-6 py-3 text-left text-xs font-semibold text-gray-700 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200">
              <tr v-for="order in orders" :key="order.id" class="hover:bg-gray-50">
                <td class="px-6 py-4 text-sm font-semibold text-gray-900">{{ order.order_number }}</td>
                <td class="px-6 py-4 text-sm text-gray-700">
                  <div>{{ order.customer_name }}</div>
                  <div class="text-xs text-gray-500">{{ order.customer_email }}</div>
                </td>
                <td class="px-6 py-4 text-sm font-semibold text-gray-900">₱{{ formatPrice(order.total_amount) }}</td>
                <td class="px-6 py-4 text-sm">
                  <span :class="[
                    'px-3 py-1 rounded-full text-xs font-semibold',
                    order.payment_status === 'paid'
                      ? 'bg-green-100 text-green-800'
                      : 'bg-yellow-100 text-yellow-800'
                  ]">
                    {{ order.payment_status }}
                  </span>
                </td>
                <td class="px-6 py-4 text-sm">
                  <select
                    :value="order.order_status"
                    @change="handleStatusChange(order.id, $event.target.value)"
                    :class="[
                      'px-3 py-1 rounded border',
                      order.order_status === 'delivered'
                        ? 'bg-green-50 border-green-300 text-green-800'
                        : order.order_status === 'shipped'
                        ? 'bg-blue-50 border-blue-300 text-blue-800'
                        : order.order_status === 'cancelled'
                        ? 'bg-red-50 border-red-300 text-red-800'
                        : 'bg-yellow-50 border-yellow-300 text-yellow-800'
                    ]"
                  >
                    <option value="pending">Pending</option>
                    <option value="processing">Processing</option>
                    <option value="shipped">Shipped</option>
                    <option value="delivered">Delivered</option>
                    <option value="cancelled">Cancelled</option>
                  </select>
                </td>
                <td class="px-6 py-4 text-sm text-gray-600">
                  {{ formatDate(order.created_at) }}
                </td>
                <td class="px-6 py-4 text-sm">
                  <button
                    @click="handleViewOrder(order)"
                    class="text-blue-600 hover:text-blue-700 font-medium"
                  >
                    View
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-12">
        <p class="text-gray-500 text-lg mb-2">No orders found</p>
        <p class="text-sm text-gray-400">Orders will appear here once customers place them</p>
      </div>

      <!-- Order Detail Modal -->
      <div v-if="selectedOrder" class="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
        <div class="bg-white rounded-lg max-w-2xl w-full max-h-96 overflow-y-auto">
          <div class="p-6 border-b border-gray-200 flex items-center justify-between">
            <h3 class="text-lg font-semibold text-gray-900">Order {{ selectedOrder.order_number }}</h3>
            <button
              @click="selectedOrder = null"
              class="text-gray-500 hover:text-gray-700 text-2xl"
            >
              ×
            </button>
          </div>

          <div class="p-6 space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-xs text-gray-600 uppercase">Status</p>
                <p class="font-semibold text-gray-900">{{ selectedOrder.order_status }}</p>
              </div>
              <div>
                <p class="text-xs text-gray-600 uppercase">Total</p>
                <p class="font-semibold text-gray-900">₱{{ formatPrice(selectedOrder.total_amount) }}</p>
              </div>
            </div>

            <div>
              <p class="text-xs text-gray-600 uppercase mb-2">Items</p>
              <div class="space-y-2 bg-gray-50 rounded p-3">
                <div v-for="item in selectedOrder.items" :key="item.id" class="flex justify-between text-sm">
                  <span class="text-gray-700">{{ item.product_name }} x{{ item.quantity }}</span>
                  <span class="font-medium text-gray-900">₱{{ formatPrice(item.total_price) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { adminOrders } from '@/utils/ecommerce'

const authStore = useAuthStore()

// State
const orders = ref([])
const selectedOrder = ref(null)
const isLoading = ref(false)
const error = ref('')

// Methods
function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

function formatDate(dateStr) {
  return new Date(dateStr).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function handleViewOrder(order) {
  selectedOrder.value = order
}

async function handleStatusChange(orderId, newStatus) {
  try {
    const result = await adminOrders.updateStatus(authStore.token, orderId, newStatus)
    const index = orders.value.findIndex(o => o.id === orderId)
    if (index > -1) {
      orders.value[index] = result
    }
  } catch (err) {
    error.value = err.message
  }
}

async function loadOrders() {
  isLoading.value = true
  error.value = ''
  try {
    const result = await adminOrders.getAll(authStore.token)
    orders.value = result || []
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadOrders()
})
</script>
