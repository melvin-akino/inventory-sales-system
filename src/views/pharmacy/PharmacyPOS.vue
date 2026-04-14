<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Pharmacy Header -->
    <div class="bg-gradient-to-r from-green-600 to-green-700 text-white p-4 sticky top-0 z-30">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">💊 Pharmacy POS</h1>
          <p class="text-green-100 text-sm">Prescription & Medication Management</p>
        </div>
        <div class="text-right">
          <p class="text-sm">{{ currentDate }}</p>
          <p class="text-green-100 text-xs">{{ authStore.user?.username }}</p>
        </div>
      </div>
    </div>

    <div class="flex h-[calc(100vh-80px)]">
      <!-- Left Sidebar - Prescription & Patient -->
      <div class="w-80 bg-white border-r border-gray-200 overflow-y-auto">
        <div class="p-4 space-y-4">
          <!-- Tabs -->
          <div class="flex gap-2">
            <button
              @click="activeTab = 'prescription'"
              :class="[
                'flex-1 py-2 px-3 rounded-lg font-medium text-sm transition-colors',
                activeTab === 'prescription'
                  ? 'bg-green-500 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              ]"
            >
              Rx Prescription
            </button>
            <button
              @click="activeTab = 'patient'"
              :class="[
                'flex-1 py-2 px-3 rounded-lg font-medium text-sm transition-colors',
                activeTab === 'patient'
                  ? 'bg-green-500 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              ]"
            >
              Patient
            </button>
          </div>

          <!-- Prescription Tab -->
          <div v-if="activeTab === 'prescription'" class="space-y-3">
            <div>
              <label class="text-sm font-medium text-gray-700">Prescription Number</label>
              <input
                v-model="selectedPrescription.prescription_number"
                type="text"
                placeholder="Search by Rx #"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>

            <div>
              <label class="text-sm font-medium text-gray-700">Doctor</label>
              <input
                v-model="selectedPrescription.doctor_name"
                type="text"
                placeholder="Doctor name"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>

            <div>
              <label class="text-sm font-medium text-gray-700">Instructions</label>
              <textarea
                v-model="selectedPrescription.instructions"
                placeholder="e.g., Take 1 tablet twice daily with food"
                rows="3"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>

            <button
              @click="loadPrescription"
              class="w-full py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 font-medium text-sm"
            >
              Load Prescription
            </button>
          </div>

          <!-- Patient Tab -->
          <div v-if="activeTab === 'patient'" class="space-y-3">
            <div>
              <label class="text-sm font-medium text-gray-700">Patient Name</label>
              <input
                v-model="selectedPatient.name"
                type="text"
                placeholder="Search or add patient"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>

            <div>
              <label class="text-sm font-medium text-gray-700">Phone</label>
              <input
                v-model="selectedPatient.phone"
                type="tel"
                placeholder="Patient phone"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>

            <div>
              <label class="text-sm font-medium text-gray-700">Allergies</label>
              <input
                v-model="selectedPatient.allergies"
                type="text"
                placeholder="e.g., Penicillin, Sulfa"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>

            <button
              @click="selectOrAddPatient"
              class="w-full py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 font-medium text-sm"
            >
              {{ selectedPatient.id ? 'Update Patient' : 'Add Patient' }}
            </button>
          </div>

          <!-- Alerts Section -->
          <div v-if="alertedMedications.length > 0" class="bg-yellow-50 border border-yellow-200 rounded-lg p-3">
            <p class="text-sm font-semibold text-yellow-900 mb-2">⚠️ Alerts</p>
            <ul class="space-y-1 text-xs text-yellow-800">
              <li v-for="alert in alertedMedications" :key="alert.id">
                {{ alert.message }}
              </li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Center - Medication Grid -->
      <div class="flex-1 overflow-y-auto bg-gray-100 p-6">
        <div class="mb-4">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search medications by name, generic name, or strength..."
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-green-500"
          />
        </div>

        <!-- Medication Grid -->
        <div v-if="filteredMedications.length > 0" class="grid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          <button
            v-for="medication in filteredMedications"
            :key="medication.id"
            @click="addMedicationToCart(medication)"
            class="bg-white rounded-lg shadow hover:shadow-lg transition-all border-2 border-transparent hover:border-green-500 p-4 text-left"
          >
            <div class="flex justify-between items-start mb-2">
              <div class="flex-1">
                <h3 class="font-semibold text-gray-900 text-sm">{{ medication.name }}</h3>
                <p class="text-xs text-gray-600">{{ medication.generic_name }}</p>
              </div>
              <span v-if="medication.requires_prescription" class="text-xs bg-red-100 text-red-800 px-2 py-1 rounded">
                Rx
              </span>
              <span v-if="medication.is_controlled_substance" class="text-xs bg-yellow-100 text-yellow-800 px-2 py-1 rounded">
                C
              </span>
            </div>

            <div class="space-y-1 mb-3">
              <div class="flex justify-between text-xs">
                <span class="text-gray-600">Strength:</span>
                <span class="font-medium">{{ medication.strength }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-gray-600">Form:</span>
                <span class="font-medium">{{ medication.dosage_form }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-gray-600">Stock:</span>
                <span :class="medication.quantity > medication.reorder_level ? 'text-green-600' : 'text-red-600'" class="font-medium">
                  {{ medication.quantity }}
                </span>
              </div>
            </div>

            <div v-if="medication.expiry_date" class="text-xs text-gray-500 mb-2">
              Expires: {{ formatDate(medication.expiry_date) }}
            </div>

            <div class="flex justify-between items-center">
              <span class="text-lg font-bold text-green-600">₱{{ medication.selling_price.toFixed(2) }}</span>
              <button
                class="px-3 py-1 bg-green-500 text-white rounded text-sm hover:bg-green-600 font-medium"
              >
                Add
              </button>
            </div>
          </button>
        </div>

        <!-- No Results -->
        <div v-else class="text-center py-12">
          <p class="text-gray-500 text-lg">No medications found</p>
        </div>
      </div>

      <!-- Right Panel - Pharmacy Cart -->
      <div class="w-96 bg-white border-l border-gray-200 flex flex-col shadow-lg">
        <!-- Header -->
        <div class="bg-gradient-to-r from-green-600 to-green-700 text-white px-4 py-3">
          <h2 class="text-lg font-bold">💊 Medication Cart</h2>
          <p class="text-xs text-green-100">{{ cartItems.length }} item(s)</p>
        </div>

        <!-- Patient Info if selected -->
        <div v-if="selectedPatient.id" class="bg-blue-50 border-b border-blue-200 p-3">
          <p class="text-sm font-semibold text-blue-900">Patient: {{ selectedPatient.name }}</p>
          <p class="text-xs text-blue-700">{{ selectedPatient.phone }}</p>
          <p v-if="selectedPatient.allergies" class="text-xs text-red-600 mt-1">⚠️ Allergies: {{ selectedPatient.allergies }}</p>
        </div>

        <!-- Cart Items -->
        <div class="flex-1 overflow-y-auto">
          <div v-if="cartItems.length > 0" class="divide-y">
            <div
              v-for="(item, idx) in cartItems"
              :key="idx"
              class="p-3 hover:bg-gray-50 transition-colors"
            >
              <div class="flex justify-between items-start mb-2">
                <div class="flex-1">
                  <h4 class="font-semibold text-gray-900 text-sm">{{ item.name }}</h4>
                  <p class="text-xs text-gray-600">{{ item.strength }} - {{ item.dosage_form }}</p>
                  <p class="text-xs text-gray-500 mt-1">₱{{ item.selling_price }} each</p>
                </div>
                <button
                  @click="removeFromCart(idx)"
                  class="text-red-500 hover:text-red-700 font-bold"
                >
                  ✕
                </button>
              </div>

              <div class="flex items-center gap-2 mb-2">
                <button
                  @click="decrementQty(idx)"
                  class="px-2 py-1 bg-gray-200 rounded text-sm font-bold"
                >
                  −
                </button>
                <input
                  :value="item.quantity"
                  @input="updateQty(idx, $event.target.value)"
                  type="number"
                  class="w-12 text-center border border-gray-300 rounded text-sm"
                />
                <button
                  @click="incrementQty(idx)"
                  class="px-2 py-1 bg-gray-200 rounded text-sm font-bold"
                >
                  +
                </button>
              </div>

              <div class="flex justify-between text-sm mb-2">
                <span class="text-gray-600">Total:</span>
                <span class="font-bold">₱{{ (item.quantity * item.selling_price).toFixed(2) }}</span>
              </div>

              <!-- Dosage Instructions for medication -->
              <div class="mt-2 pt-2 border-t border-gray-200">
                <input
                  :value="item.instructions || ''"
                  @input="updateInstructions(idx, $event.target.value)"
                  type="text"
                  placeholder="Instructions (e.g., 1 tablet daily)"
                  class="w-full px-2 py-1 border border-gray-300 rounded text-xs focus:outline-none focus:ring-2 focus:ring-green-500"
                />
              </div>
            </div>
          </div>

          <div v-else class="flex flex-col items-center justify-center h-48">
            <p class="text-gray-400 text-2xl">💊</p>
            <p class="text-gray-500 text-sm mt-2">No medications added</p>
          </div>
        </div>

        <!-- Totals Section -->
        <div class="border-t bg-gray-50 p-3 space-y-2">
          <div class="flex justify-between text-sm">
            <span>Subtotal:</span>
            <span class="font-semibold">₱{{ subtotal.toFixed(2) }}</span>
          </div>

          <div class="space-y-1">
            <label class="text-xs font-medium text-gray-700">Discount:</label>
            <div class="flex gap-2">
              <input
                v-model.number="discountPercent"
                type="number"
                min="0"
                max="100"
                class="w-16 px-2 py-1 border border-gray-300 rounded text-xs"
              />
              <span class="text-xs">%</span>
              <input
                v-model.number="discountAmount"
                type="number"
                class="flex-1 px-2 py-1 border border-gray-300 rounded text-xs"
              />
              <span class="text-xs">₱</span>
            </div>
          </div>

          <div class="flex justify-between text-sm border-t pt-2">
            <span>VAT (12%):</span>
            <span class="font-semibold">₱{{ vatAmount.toFixed(2) }}</span>
          </div>

          <div class="flex justify-between text-lg bg-green-50 p-2 rounded border border-green-200">
            <span class="font-bold">TOTAL:</span>
            <span class="font-bold text-green-600">₱{{ totalAmount.toFixed(2) }}</span>
          </div>

          <!-- Payment -->
          <div class="space-y-2 pt-2">
            <label class="text-xs font-medium text-gray-700">Payment:</label>
            <select v-model="paymentMethod" class="w-full px-2 py-1 border border-gray-300 rounded text-xs">
              <option value="cash">💵 Cash</option>
              <option value="card">💳 Card</option>
              <option value="insurance">🏥 Insurance</option>
              <option value="mixed">Mixed</option>
            </select>

            <label class="text-xs font-medium text-gray-700">Amount Paid:</label>
            <input
              v-model.number="amountPaid"
              type="number"
              placeholder="0.00"
              class="w-full px-2 py-1 border border-gray-300 rounded text-lg font-bold focus:outline-none focus:ring-2 focus:ring-green-500"
            />

            <div class="bg-green-50 p-2 rounded border border-green-200">
              <p class="text-xs text-green-700">Change:</p>
              <p class="text-xl font-bold text-green-600">₱{{ Math.max(0, amountPaid - totalAmount).toFixed(2) }}</p>
            </div>
          </div>

          <!-- Complete Sale Button -->
          <button
            v-if="cartItems.length > 0"
            @click="completeSale"
            :disabled="amountPaid < totalAmount"
            :class="[
              'w-full py-3 rounded-lg font-bold text-lg transition-all duration-200 mt-3',
              amountPaid >= totalAmount
                ? 'bg-green-500 hover:bg-green-600 text-white cursor-pointer'
                : 'bg-gray-300 text-gray-500 cursor-not-allowed'
            ]"
          >
            ✓ Complete Sale
          </button>

          <!-- Print Receipt -->
          <button
            v-if="cartItems.length > 0"
            @click="printReceipt"
            class="w-full py-2 rounded-lg font-medium text-sm text-gray-700 bg-gray-200 hover:bg-gray-300 transition-colors"
          >
            🖨️ Print Receipt
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { productsApi } from '@/utils/api'

const authStore = useAuthStore()

// State
const activeTab = ref('patient')
const medications = ref([])
const cartItems = ref([])
const searchQuery = ref('')
const currentDate = ref(new Date().toLocaleDateString())

// Prescription state
const selectedPrescription = ref({
  prescription_number: '',
  doctor_name: '',
  instructions: '',
})

// Patient state
const selectedPatient = ref({
  id: null,
  name: '',
  phone: '',
  allergies: '',
})

// Payment state
const discountPercent = ref(0)
const discountAmount = ref(0)
const paymentMethod = ref('cash')
const amountPaid = ref(0)

// Computed
const filteredMedications = computed(() => {
  if (!searchQuery.value) return medications.value
  
  const q = searchQuery.value.toLowerCase()
  return medications.value.filter(m =>
    m.name.toLowerCase().includes(q) ||
    m.generic_name?.toLowerCase().includes(q) ||
    m.strength?.toLowerCase().includes(q)
  )
})

const subtotal = computed(() =>
  cartItems.value.reduce((sum, item) => sum + item.quantity * item.selling_price, 0)
)

const totalDiscount = computed(() => {
  const percentDiscount = subtotal.value * (discountPercent.value / 100)
  return percentDiscount + (discountAmount.value || 0)
})

const vatAmount = computed(() => (subtotal.value - totalDiscount.value) * 0.12)

const totalAmount = computed(() => subtotal.value - totalDiscount.value + vatAmount.value)

const alertedMedications = computed(() => {
  const alerts = []
  medications.value.forEach(med => {
    if (med.quantity < med.reorder_level) {
      alerts.push({
        id: med.id,
        message: `${med.name} low stock (${med.quantity} remaining)`
      })
    }
    if (med.expiry_date) {
      const expiryDate = new Date(med.expiry_date)
      const daysUntilExpiry = Math.floor((expiryDate - new Date()) / (1000 * 60 * 60 * 24))
      if (daysUntilExpiry < 30 && daysUntilExpiry > 0) {
        alerts.push({
          id: med.id,
          message: `${med.name} expires in ${daysUntilExpiry} days`
        })
      } else if (daysUntilExpiry <= 0) {
        alerts.push({
          id: med.id,
          message: `${med.name} EXPIRED`
        })
      }
    }
  })
  return alerts
})

// Methods
function addMedicationToCart(medication) {
  const existing = cartItems.value.find(item => item.id === medication.id)
  if (existing) {
    existing.quantity += 1
  } else {
    cartItems.value.push({
      ...medication,
      quantity: 1,
      instructions: ''
    })
  }
}

function removeFromCart(idx) {
  cartItems.value.splice(idx, 1)
}

function incrementQty(idx) {
  const item = cartItems.value[idx]
  if (item.quantity < item.quantity_on_hand) {
    item.quantity += 1
  }
}

function decrementQty(idx) {
  if (cartItems.value[idx].quantity > 1) {
    cartItems.value[idx].quantity -= 1
  }
}

function updateQty(idx, value) {
  cartItems.value[idx].quantity = Math.max(1, parseInt(value) || 1)
}

function updateInstructions(idx, value) {
  cartItems.value[idx].instructions = value
}

function loadPrescription() {
  // TODO: Implement prescription loading from API
  alert('Load prescription from database')
}

function selectOrAddPatient() {
  // TODO: Implement patient selection/creation
  alert('Patient selected/created')
}

async function completeSale() {
  if (cartItems.value.length === 0 || amountPaid.value < totalAmount.value) {
    alert('Cannot complete sale')
    return
  }

  try {
    // TODO: Create pharmacy sale record
    alert('Sale completed!')
    resetCart()
  } catch (err) {
    alert(`Error: ${err.message}`)
  }
}

function printReceipt() {
  // TODO: Implement receipt printing
  alert('Receipt printed')
}

function resetCart() {
  cartItems.value = []
  amountPaid.value = 0
  discountPercent.value = 0
  discountAmount.value = 0
}

function formatDate(dateString) {
  return new Date(dateString).toLocaleDateString()
}

// Load medications on mount
async function loadMedications() {
  try {
    const data = await productsApi.getProducts(authStore.token, null)
    medications.value = data.products || data || []
  } catch (err) {
    console.error('Error loading medications:', err)
  }
}

onMounted(() => {
  loadMedications()
})
</script>
