<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center">
        <h1 class="text-2xl font-bold text-blue-600">Payment Processing</h1>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="currentOrder" class="space-y-6">
        <!-- Order Details Card -->
        <div class="bg-white rounded-lg border border-gray-200 p-6">
          <div class="flex items-center justify-between mb-6">
            <div>
              <h2 class="text-lg font-semibold text-gray-900">Order #{{ currentOrder.order_number }}</h2>
              <p class="text-sm text-gray-600 mt-1">{{ new Date(currentOrder.created_at).toLocaleString() }}</p>
            </div>
            <div class="text-right">
              <p class="text-sm text-gray-600">Total Amount</p>
              <p class="text-3xl font-bold text-blue-600">₱{{ formatPrice(currentOrder.total_amount) }}</p>
            </div>
          </div>

          <!-- Order Items -->
          <div class="border-t border-gray-200 pt-4">
            <div v-for="item in currentOrder.items" :key="item.id" class="flex justify-between text-sm mb-2">
              <span class="text-gray-700">{{ item.product_name }} x{{ item.quantity }}</span>
              <span class="text-gray-900 font-medium">₱{{ formatPrice(item.total_price) }}</span>
            </div>
          </div>
        </div>

        <!-- Payment Processing Card -->
        <div class="bg-white rounded-lg border border-gray-200 p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-6">Payment Method</h3>

          <div class="grid grid-cols-2 gap-4 mb-6">
            <button
              v-for="method in paymentMethods"
              :key="method.value"
              @click="selectedPaymentMethod = method.value"
              :class="[
                'p-4 border-2 rounded-lg transition-all',
                selectedPaymentMethod === method.value
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200 hover:border-gray-300'
              ]"
            >
              <div class="text-2xl mb-2">{{ method.icon }}</div>
              <div class="text-sm font-medium text-gray-900">{{ method.label }}</div>
            </button>
          </div>

          <!-- Mock Payment Simulation -->
          <div class="mb-6 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
            <p class="text-sm text-yellow-800">
              <strong>Demo Mode:</strong> This is a mock payment gateway. Click "Process Payment" to simulate a payment transaction (95% success rate).
            </p>
          </div>

          <!-- Processing Status -->
          <div v-if="isProcessing" class="mb-6 p-4 bg-blue-50 border border-blue-200 rounded-lg">
            <div class="flex items-center gap-2">
              <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-500"></div>
              <span class="text-sm text-blue-700">Processing payment...</span>
            </div>
          </div>

          <!-- Payment Result -->
          <div v-if="paymentResult" :class="[
            'mb-6 p-4 rounded-lg border',
            paymentResult.success
              ? 'bg-green-50 border-green-200'
              : 'bg-red-50 border-red-200'
          ]">
            <p :class="[
              'text-sm font-medium',
              paymentResult.success
                ? 'text-green-800'
                : 'text-red-800'
            ]">
              {{ paymentResult.success ? '✓ Payment Successful!' : '✗ Payment Failed' }}
            </p>
            <p v-if="paymentResult.transaction_id" class="text-xs text-gray-600 mt-1">
              Transaction ID: {{ paymentResult.transaction_id }}
            </p>
          </div>

          <!-- Action Buttons -->
          <div class="flex gap-3">
            <button
              @click="handleProcessPayment"
              :disabled="isProcessing || paymentResult"
              :class="[
                'flex-1 px-6 py-3 rounded-lg font-semibold text-white',
                isProcessing || paymentResult
                  ? 'bg-gray-400 cursor-not-allowed'
                  : 'bg-green-600 hover:bg-green-700'
              ]"
            >
              Process Payment
            </button>
            <button
              @click="handleRetry"
              v-if="!paymentResult?.success"
              class="px-6 py-3 border border-gray-300 text-gray-700 rounded-lg font-semibold hover:bg-gray-50"
            >
              Try Again
            </button>
          </div>
        </div>

        <!-- Next Steps -->
        <div v-if="paymentResult?.success" class="bg-green-50 border border-green-200 rounded-lg p-6">
          <h3 class="text-lg font-semibold text-green-900 mb-3">What's Next?</h3>
          <ul class="space-y-2 text-sm text-green-800">
            <li>✓ Your payment has been processed successfully</li>
            <li>✓ A confirmation email will be sent to {{ currentOrder.customer_name }}</li>
            <li>✓ Your order will be prepared for shipment</li>
            <li>✓ You can track your order status in your account</li>
          </ul>
          <router-link
            to="/order-confirmation"
            class="inline-block mt-4 px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 font-medium"
          >
            View Order Confirmation
          </router-link>
        </div>
      </div>

      <div v-else class="text-center py-12">
        <p class="text-gray-600">No order to process</p>
        <router-link to="/shop" class="inline-block mt-4 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
          Back to Shop
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useEcommerceStore } from '@/stores/ecommerce'

const router = useRouter()
const ecommerceStore = useEcommerceStore()

// State
const selectedPaymentMethod = ref('card')
const isProcessing = ref(false)
const paymentResult = ref(null)

// Computed
const currentOrder = computed(() => ecommerceStore.currentOrder)

const paymentMethods = [
  { value: 'card', label: 'Credit/Debit Card', icon: '💳' },
  { value: 'gcash', label: 'GCash', icon: '📱' },
  { value: 'maya', label: 'Maya', icon: '📱' },
  { value: 'bank_transfer', label: 'Bank Transfer', icon: '🏦' },
]

// Methods
function formatPrice(price) {
  return parseFloat(price || 0).toFixed(2)
}

async function handleProcessPayment() {
  if (!currentOrder.value) {
    return
  }

  isProcessing.value = true
  paymentResult.value = null

  try {
    const result = await ecommerceStore.processPayment(selectedPaymentMethod.value)
    paymentResult.value = {
      success: result.success,
      transaction_id: result.transaction_id,
      payment_status: result.payment_status,
    }

    if (result.success) {
      setTimeout(() => {
        router.push('/order-confirmation')
      }, 2000)
    }
  } catch (err) {
    paymentResult.value = {
      success: false,
      error: err.message,
    }
  } finally {
    isProcessing.value = false
  }
}

function handleRetry() {
  paymentResult.value = null
  isProcessing.value = false
}
</script>
