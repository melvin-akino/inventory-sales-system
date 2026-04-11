<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Products</h2>
      <button v-if="can(MANAGE)" class="btn-primary" @click="openCreate">+ Add Product</button>
    </div>

    <!-- Filters -->
    <div class="card mb-4">
      <div class="flex flex-wrap gap-3">
        <input v-model="filter.search" class="input w-60" placeholder="🔍 Search name or SKU…" @input="loadProducts" />
        <select v-model="filter.category_id" class="input w-44" @change="loadProducts">
          <option :value="null">All Categories</option>
          <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
        <label class="flex items-center gap-2 text-sm text-gray-600 cursor-pointer">
          <input type="checkbox" v-model="filter.low_stock_only" @change="loadProducts" class="w-4 h-4 rounded" />
          Low Stock Only
        </label>
        <label class="flex items-center gap-2 text-sm text-gray-600 cursor-pointer">
          <input type="checkbox" v-model="showInactive" @change="loadProducts" class="w-4 h-4 rounded" />
          Show Inactive
        </label>
      </div>
    </div>

    <!-- Table -->
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>SKU</th>
            <th>Product</th>
            <th>Category</th>
            <th>Unit Price</th>
            <th>Stock</th>
            <th>Status</th>
            <th v-if="can(MANAGE)">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="loading"><td colspan="7" class="text-center py-8 text-gray-400">Loading…</td></tr>
          <tr v-else-if="!products.length"><td colspan="7" class="text-center py-8 text-gray-400">No products found</td></tr>
          <tr v-for="p in products" :key="p.id">
            <td class="font-mono text-xs">{{ p.sku }}</td>
            <td>
              <p class="font-medium text-gray-900">{{ p.name }}</p>
              <p class="text-xs text-gray-400">{{ p.description }}</p>
            </td>
            <td class="text-gray-500">{{ p.category_name ?? '—' }}</td>
            <td class="font-semibold">{{ formatCurrency(p.selling_price) }}</td>
            <td>
              <div class="flex items-center gap-2">
                <span class="font-semibold">{{ p.quantity }}</span>
                <span class="text-xs text-gray-400">{{ p.unit }}</span>
              </div>
            </td>
            <td>
              <span class="badge" :class="stockStatusColor(p.quantity, p.reorder_level)">
                {{ stockStatusLabel(p.quantity, p.reorder_level) }}
              </span>
            </td>
            <td v-if="can(MANAGE)">
              <div class="flex items-center gap-1">
                <button class="btn btn-sm btn-secondary" @click="openEdit(p)">Edit</button>
                <button class="btn btn-sm bg-yellow-50 text-yellow-700 border border-yellow-200 hover:bg-yellow-100" @click="openAdjust(p)">Stock</button>
                <button class="btn btn-sm btn-danger" @click="confirmDelete(p)">Delete</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Create/Edit Modal -->
    <Modal v-model="showModal" :title="editingProduct ? 'Edit Product' : 'Add Product'" width="640px">
      <form @submit.prevent="saveProduct" class="grid grid-cols-2 gap-4">
        <div class="col-span-2">
          <label class="label">Product Name *</label>
          <input v-model="form.name" class="input" required placeholder="e.g. LED Bulb 9W Daylight" />
        </div>
        <div>
          <label class="label">SKU *</label>
          <input v-model="form.sku" class="input" required placeholder="e.g. LED-9W-DL" />
        </div>
        <div>
          <label class="label">Category</label>
          <select v-model="form.category_id" class="input">
            <option :value="null">— None —</option>
            <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>
        <div>
          <label class="label">Unit of Measure</label>
          <select v-model="form.unit" class="input">
            <option>piece</option><option>box</option><option>set</option><option>meter</option><option>roll</option>
          </select>
        </div>
        <div>
          <label class="label">Reorder Level</label>
          <input v-model.number="form.reorder_level" type="number" min="0" class="input" />
        </div>
        <div>
          <label class="label">Cost Price (₱) *</label>
          <input v-model.number="form.cost_price" type="number" min="0" step="0.01" class="input" required />
        </div>
        <div>
          <label class="label">Selling Price (₱) *</label>
          <input v-model.number="form.selling_price" type="number" min="0" step="0.01" class="input" required />
        </div>
        <div v-if="!editingProduct">
          <label class="label">Initial Stock</label>
          <input v-model.number="form.quantity" type="number" min="0" class="input" />
        </div>
        <div class="col-span-2">
          <label class="label">Description</label>
          <input v-model="form.description" class="input" placeholder="Optional description" />
        </div>
        <div class="col-span-2 flex items-center gap-2">
          <input type="checkbox" v-model="form.is_vat_exempt" id="vat_exempt" class="w-4 h-4" />
          <label for="vat_exempt" class="text-sm text-gray-700">VAT Exempt</label>
        </div>
        <div v-if="formError" class="col-span-2 text-sm text-red-600">{{ formError }}</div>
        <div class="col-span-2 flex justify-end gap-3">
          <button type="button" class="btn-secondary" @click="showModal = false">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save Product' }}</button>
        </div>
      </form>
    </Modal>

    <!-- Stock Adjustment Modal -->
    <Modal v-model="showAdjustModal" title="Adjust Stock" width="400px">
      <p class="text-sm text-gray-600 mb-4">Product: <strong>{{ adjustingProduct?.name }}</strong> (Current: {{ adjustingProduct?.quantity }})</p>
      <form @submit.prevent="saveAdjustment" class="space-y-4">
        <div>
          <label class="label">Adjustment Type</label>
          <select v-model="adjustForm.adjustment_type" class="input">
            <option value="add">Add Stock</option>
            <option value="subtract">Remove Stock</option>
            <option value="set">Set to Exact</option>
          </select>
        </div>
        <div>
          <label class="label">Quantity</label>
          <input v-model.number="adjustForm.quantity" type="number" min="1" class="input" required />
        </div>
        <div>
          <label class="label">Reason</label>
          <input v-model="adjustForm.reason" class="input" placeholder="e.g. Received from supplier" />
        </div>
        <div v-if="formError" class="text-sm text-red-600">{{ formError }}</div>
        <div class="flex justify-end gap-3">
          <button type="button" class="btn-secondary" @click="showAdjustModal = false">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Apply' }}</button>
        </div>
      </form>
    </Modal>

    <ConfirmDialog
      v-model="showDeleteDialog"
      title="Delete Product"
      :message="`Deactivate '${deletingProduct?.name}'? It will no longer appear in active listings.`"
      confirm-label="Delete"
      :danger="true"
      @confirm="deleteProduct"
    />
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { productsApi, categoriesApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatCurrency, stockStatusColor, stockStatusLabel, ACCESS } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'

const auth = useAuthStore()
const appStore = useAppStore()
const { MANAGE } = ACCESS
const can = (r) => auth.can(r)

const products = ref([])
const categories = ref([])
const loading = ref(false)
const showModal = ref(false)
const showAdjustModal = ref(false)
const showDeleteDialog = ref(false)
const saving = ref(false)
const formError = ref('')
const editingProduct = ref(null)
const adjustingProduct = ref(null)
const deletingProduct = ref(null)
const showInactive = ref(false)

const filter = reactive({ search: '', category_id: null, low_stock_only: false })

const defaultForm = () => ({
  category_id: null, sku: '', name: '', description: '', unit: 'piece',
  cost_price: 0, selling_price: 0, quantity: 0, reorder_level: 10, is_vat_exempt: false,
})
const form = ref(defaultForm())
const adjustForm = ref({ adjustment_type: 'add', quantity: 1, reason: '' })

async function loadProducts() {
  loading.value = true
  try {
    products.value = await productsApi.getProducts(auth.token, {
      search: filter.search || null,
      category_id: filter.category_id,
      low_stock_only: filter.low_stock_only,
      active_only: !showInactive.value,
    })
  } catch (e) {
    appStore.notify(e.message, 'error')
  } finally {
    loading.value = false
  }
}

async function loadCategories() {
  try { categories.value = await categoriesApi.getCategories(auth.token) } catch (_) {}
}

function openCreate() {
  editingProduct.value = null
  form.value = defaultForm()
  formError.value = ''
  showModal.value = true
}

function openEdit(p) {
  editingProduct.value = p
  form.value = { ...p, is_vat_exempt: !!p.is_vat_exempt }
  formError.value = ''
  showModal.value = true
}

function openAdjust(p) {
  adjustingProduct.value = p
  adjustForm.value = { adjustment_type: 'add', quantity: 1, reason: '' }
  formError.value = ''
  showAdjustModal.value = true
}

function confirmDelete(p) {
  deletingProduct.value = p
  showDeleteDialog.value = true
}

async function saveProduct() {
  saving.value = true
  formError.value = ''
  try {
    if (editingProduct.value) {
      await productsApi.updateProduct(auth.token, editingProduct.value.id, form.value)
      appStore.notify('Product updated')
    } else {
      await productsApi.createProduct(auth.token, form.value)
      appStore.notify('Product created')
    }
    showModal.value = false
    loadProducts()
  } catch (e) {
    formError.value = e.message
  } finally {
    saving.value = false
  }
}

async function saveAdjustment() {
  saving.value = true
  formError.value = ''
  try {
    await productsApi.adjustStock(auth.token, {
      product_id: adjustingProduct.value.id,
      adjustment_type: adjustForm.value.adjustment_type,
      quantity: adjustForm.value.quantity,
      reason: adjustForm.value.reason || null,
    })
    appStore.notify('Stock adjusted successfully')
    showAdjustModal.value = false
    loadProducts()
  } catch (e) {
    formError.value = e.message
  } finally {
    saving.value = false
  }
}

async function deleteProduct() {
  try {
    await productsApi.deleteProduct(auth.token, deletingProduct.value.id)
    appStore.notify('Product deactivated')
    loadProducts()
  } catch (e) {
    appStore.notify(e.message, 'error')
  }
}

onMounted(() => { loadProducts(); loadCategories() })
</script>
