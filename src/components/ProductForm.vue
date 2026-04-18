<template>
  <div class="space-y-6">
    <!-- Product Basic Info -->
    <div class="bg-white rounded-lg shadow p-6 space-y-4">
      <h3 class="text-lg font-semibold text-gray-900">Product Information</h3>

      <div class="grid grid-cols-2 gap-4">
        <!-- SKU -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">SKU *</label>
          <input
            v-model="form.sku"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="e.g., LED-12W-DL"
            required
          />
        </div>

        <!-- Product Name -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Product Name *</label>
          <input
            v-model="form.name"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="e.g., LED Bulb 12W Daylight"
            required
          />
        </div>
      </div>

      <!-- Description -->
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">Description</label>
        <textarea
          v-model="form.description"
          rows="3"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
          placeholder="Product description..."
        />
      </div>

      <!-- Category & Unit -->
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Category</label>
          <select
            v-model="form.category_id"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
          >
            <option :value="null">-- Select Category --</option>
            <option v-for="cat in categories" :key="cat.id" :value="cat.id">
              {{ cat.name }}
            </option>
          </select>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Unit</label>
          <select
            v-model="form.unit"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
          >
            <option value="piece">Piece</option>
            <option value="box">Box</option>
            <option value="pack">Pack</option>
            <option value="roll">Roll</option>
            <option value="meter">Meter</option>
            <option value="liter">Liter</option>
            <option value="gram">Gram</option>
            <option value="kg">Kilogram</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Pricing & Stock -->
    <div class="bg-white rounded-lg shadow p-6 space-y-4">
      <h3 class="text-lg font-semibold text-gray-900">Pricing & Stock</h3>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Cost Price (₱)</label>
          <input
            v-model.number="form.cost_price"
            type="number"
            step="0.01"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="0.00"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Selling Price (₱) *</label>
          <input
            v-model.number="form.selling_price"
            type="number"
            step="0.01"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="0.00"
            required
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Quantity</label>
          <input
            v-model.number="form.quantity"
            type="number"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="0"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Reorder Level</label>
          <input
            v-model.number="form.reorder_level"
            type="number"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="10"
          />
        </div>
      </div>

      <!-- VAT Exempt -->
      <label class="flex items-center gap-2 cursor-pointer">
        <input
          v-model="form.is_vat_exempt"
          type="checkbox"
          class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        />
        <span class="text-sm font-medium text-gray-700">VAT Exempt</span>
      </label>
    </div>

    <!-- Industry-Specific Attributes -->
    <div v-if="industryAttributes.length > 0" class="bg-white rounded-lg shadow p-6 space-y-4">
      <h3 class="text-lg font-semibold text-gray-900">
        <span :style="{ color: industryColor }">{{ industryName }}</span>
        Specific Details
      </h3>

      <div class="space-y-4">
        <div v-for="attr in industryAttributes" :key="attr.id">
          <!-- Text Input -->
          <div v-if="attr.attribute_type === 'text'">
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ attr.attribute_label }}
              <span v-if="attr.is_required" class="text-red-500">*</span>
            </label>
            <input
              v-model="customFields[attr.attribute_name]"
              type="text"
              :required="attr.is_required"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
              :placeholder="`Enter ${attr.attribute_label.toLowerCase()}`"
            />
          </div>

          <!-- Number Input -->
          <div v-else-if="attr.attribute_type === 'number'">
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ attr.attribute_label }}
              <span v-if="attr.is_required" class="text-red-500">*</span>
            </label>
            <input
              v-model.number="customFields[attr.attribute_name]"
              type="number"
              step="0.01"
              :required="attr.is_required"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
              :placeholder="`Enter ${attr.attribute_label.toLowerCase()}`"
            />
          </div>

          <!-- Date Input -->
          <div v-else-if="attr.attribute_type === 'date'">
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ attr.attribute_label }}
              <span v-if="attr.is_required" class="text-red-500">*</span>
            </label>
            <input
              v-model="customFields[attr.attribute_name]"
              type="date"
              :required="attr.is_required"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            />
          </div>

          <!-- Textarea -->
          <div v-else-if="attr.attribute_type === 'textarea'">
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ attr.attribute_label }}
              <span v-if="attr.is_required" class="text-red-500">*</span>
            </label>
            <textarea
              v-model="customFields[attr.attribute_name]"
              rows="2"
              :required="attr.is_required"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
              :placeholder="`Enter ${attr.attribute_label.toLowerCase()}`"
            />
          </div>

          <!-- Select Dropdown -->
          <div v-else-if="attr.attribute_type === 'select'">
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ attr.attribute_label }}
              <span v-if="attr.is_required" class="text-red-500">*</span>
            </label>
            <select
              v-model="customFields[attr.attribute_name]"
              :required="attr.is_required"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            >
              <option :value="null">-- Select --</option>
              <option v-for="option in getSelectOptions(attr.attribute_name)" :key="option" :value="option">
                {{ option }}
              </option>
            </select>
          </div>

          <!-- Checkbox -->
          <div v-else-if="attr.attribute_type === 'checkbox'">
            <label class="flex items-center gap-2 cursor-pointer">
              <input
                v-model="customFields[attr.attribute_name]"
                type="checkbox"
                class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span class="text-sm font-medium text-gray-700">{{ attr.attribute_label }}</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex justify-end gap-3">
      <button
        @click="$emit('cancel')"
        class="px-4 py-2 text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition"
      >
        Cancel
      </button>
      <button
        @click="handleSubmit"
        :disabled="loading"
        class="px-4 py-2 text-white rounded-lg transition disabled:opacity-50"
        :style="{ backgroundColor: industryColor }"
      >
        {{ loading ? 'Saving...' : editingProduct ? 'Update Product' : 'Create Product' }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'
import { productsApi, categoriesApi } from '@/utils/api'

const props = defineProps({
  editingProduct: {
    type: Object,
    default: null,
  },
})

const emit = defineEmits(['cancel', 'success'])

const auth = useAuthStore()
const industry = useIndustryStore()

const form = ref({
  sku: '',
  name: '',
  description: '',
  category_id: null,
  unit: 'piece',
  cost_price: 0,
  selling_price: 0,
  quantity: 0,
  reorder_level: 10,
  is_vat_exempt: false,
})

const customFields = ref({})
const categories = ref([])
const loading = ref(false)

const industryAttributes = computed(() => industry.getIndustryAttributes())
const industryColor = computed(() => industry.industryColor)
const industryName = computed(() => industry.industryName)

// Get select options from local config or hardcoded values
function getSelectOptions(attributeName) {
  const localFields = industry.getCustomFields()
  if (localFields[attributeName]?.options) {
    return localFields[attributeName].options
  }

  // Fallback based on common attribute names
  const commonOptions = {
    colorTemp: ['Daylight', 'Warm White', 'Cool White', 'RGB'],
    dosageForm: ['Tablet', 'Capsule', 'Liquid', 'Injection', 'Cream', 'Ointment', 'Lotion'],
    size: ['XS', 'S', 'M', 'L', 'XL', 'XXL'],
    skinType: ['All', 'Dry', 'Oily', 'Combination', 'Sensitive'],
  }

  return commonOptions[attributeName] || []
}

onMounted(async () => {
  // Load categories for this industry
  try {
    categories.value = await categoriesApi.getCategories(auth.token)
  } catch (e) {
    console.error('Failed to load categories:', e)
  }

  // If editing, populate form with existing data
  if (props.editingProduct) {
    form.value = {
      sku: props.editingProduct.sku,
      name: props.editingProduct.name,
      description: props.editingProduct.description || '',
      category_id: props.editingProduct.category_id,
      unit: props.editingProduct.unit || 'piece',
      cost_price: props.editingProduct.cost_price,
      selling_price: props.editingProduct.selling_price,
      quantity: props.editingProduct.quantity,
      reorder_level: props.editingProduct.reorder_level,
      is_vat_exempt: props.editingProduct.is_vat_exempt,
    }
    // TODO: Load custom field values from backend
  }
})

async function handleSubmit() {
  loading.value = true
  try {
    const request = { ...form.value }

    if (props.editingProduct) {
      await productsApi.updateProduct(auth.token, props.editingProduct.id, request)
    } else {
      await productsApi.createProduct(auth.token, request)
    }

    emit('success')
  } catch (e) {
    console.error('Save failed:', e)
  } finally {
    loading.value = false
  }
}
</script>
