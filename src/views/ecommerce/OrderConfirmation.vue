<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center">
        <h1 class="text-2xl font-bold text-gray-900">Order Confirmation</h1>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="currentOrder" class="space-y-6">
        <!-- Success Banner -->
        <div class="bg-green-50 border border-green-200 rounded-lg p-6">
          <div class="flex items-center gap-3 mb-2">
            <div class="text-3xl">✓</div>
            <h2 class="text-2xl font-bold text-green-900">Order Confirmed!</h2>
          </div>
          <p class="text-green-700">Thank you for your purchase. Your order has been received and is being processed.</p>
        </div>

        <!-- Order Details -->
        <div class="bg-white rounded-lg border border-gray-200 p-6">
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
            <div>
              <p class="text-sm text-gray-600">Order Number</p>
              <p class="font-semibold text-lg text-gray-900">{{ currentOrder.order_number }}</p>
            </div>
            <div>
              <p class="text-sm text-gray-600">Order Date</p>
              <p class="font-semibold text-lg text-gray-900">{{ formatDate(currentOrder.created_at) }}</p>
            </div>
            <div>
              <p class="text-sm text-gray-600">Order Status</p>
              <p :class="[
                'font-semibold text-lg',
                'text-yellow-600'
              ]">{{ currentOrder.order_status }}</p>
            </div>
            <div>
              <p class="text-sm text-gray-600">Payment Status</p>
              <p :class="[
                'font-semibold text-lg',
                currentOrder.payment_status === 'paid' ? 'text-green-600' : 'text-yellow-600'
              ]">{{ currentOrder.payment_status }}</p>
            </div>
          </div>
        </div>

        <!-- Order Items -->
        <div class="bg-white rounded-lg border border-gray-200 p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Order Items</h3>
          <div class="divide-y divide-gray-200">
            <div v-for="item in currentOrder.items" :key="item.id" class="py-4 flex justify-between">
              <div>
                <p class="font-medium text-gray-900">{{ item.product_name }}</p>
                <p class="text-sm text-gray-600">Quantity: {{ item.quantity }}</p>
              </div>
              <div class="text-right">
                <p class="font-semibold text-gray-900">₱{{ formatPrice(item.total_price) }}</p>
                <p class="text-sm text-gray-600">₱{{ formatPrice(item.unit_price) }} each</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Summary -->
        <div class="bg-white rounded-lg border border-gray-200 p-6">
          <div class="space-y-3 mb-4">
            <div class="flex justify-between">
              <span class="text-gray-600">Subtotal</span>
              <span class="text-gray-900">₱{{ formatPrice(currentOrder.subtotal) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Tax (12%)</span>
              <span class="text-gray-900">₱{{ formatPrice(currentOrder.tax_amount) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Shipping</span>
              <span class="text-gray-900">₱{{ formatPrice(currentOrder.shipping_cost) }}</span>
            </div>
          </div>
          <div class="border-t border-gray-200 pt-4">
            <div class="flex justify-between">
              <span class="font-semibold text-gray-900">Total</span>
              <span class="text-2xl font-bold text-blue-600">₱{{ formatPrice(currentOrder.total_amount) }}</span>
            </div>
          </div>
        </div>

        <!-- Shipping Address -->
        <div class="bg-white rounded-lg border border-gray-200 p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Shipping Address</h3>
          <p class="text-gray-700">{{ currentOrder.shipping_address }}</p>
        </div>

        <!-- Next Steps -->
        <div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
          <h3 class="text-lg font-semibold text-blue-900 mb-3">What Happens Next?</h3>
          <ol class="space-y-2 text-sm text-blue-800">
            <li class="flex gap-2">
              <span>1.</span>
              <span>A confirmation email will be sent to your email address</span>
            </li>
            <li class="flex gap-2">
              <span>2.</span>
              <span>Your order will be processed and prepared for shipment</span>
            </li>
            <li class="flex gap-2">
              <span>3.</span>
              <span>You'll receive a tracking number once your order ships</span>
            </li>
            <li class="flex gap-2">
              <span>4.</span>
              <span>Track your order status anytime from your account</span>
            </li>
          </ol>
        </div>

        <!-- Action Buttons -->
        <div class="flex gap-4">
          <router-link
            to="/shop"
            class="flex-1 px-6 py-3 bg-blue-600 text-white rounded-lg font-semibold text-center hover:bg-blue-700"
          >
            Continue Shopping
          </router-link>
          <button
            @click="handlePrint"
            class="flex-1 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg font-semibold hover:bg-gray-50"
          >
            Print Receipt
          </button>
        </div>
      </div>

      <div v-else class="text-center py-12">
        <p class="text-gray-600 mb-4">No order to display</p>
        <router-link to="/shop" class="inline-block px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
          Back to Shop
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useEcommerceStore } from '@/stores/ecommerce'

const ecommerceStore = useEcommerceStore()

// Computed
const currentOrder = computed(() => ecommerceStore.currentOrder)

// Methods
function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

function formatDate(dateStr) {
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function handlePrint() {
  window.print()
}
</script>
