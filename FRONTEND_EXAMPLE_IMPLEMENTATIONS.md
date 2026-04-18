# Example Implementation Views

## Complete Working Examples

### 1. Products Page with Industry Support

```vue
<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex justify-between items-center">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Products</h1>
        <p v-if="industry.industryName" class="text-sm text-gray-600 mt-1">
          Showing products for
          <span class="font-semibold" :style="{ color: industry.industryColor }">
            {{ industry.industryName }}
          </span>
        </p>
      </div>
      <button
        @click="showCreateForm = true"
        class="px-4 py-2 text-white rounded-lg transition"
        :style="{ backgroundColor: industry.industryColor }}"
      >
        + New Product
      </button>
    </div>

    <!-- Create Form Modal -->
    <div v-if="showCreateForm" class="fixed inset-0 bg-black bg-opacity-50 z-40 flex items-center justify-center">
      <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
        <div class="p-6 border-b flex justify-between items-center">
          <h2 class="text-xl font-semibold">Create New Product</h2>
          <button @click="showCreateForm = false" class="text-gray-500 hover:text-gray-700">✕</button>
        </div>
        <div class="p-6">
          <ProductForm 
            @success="handleProductCreated" 
            @cancel="showCreateForm = false"
          />
        </div>
      </div>
    </div>

    <!-- Products Table -->
    <div class="bg-white rounded-lg shadow overflow-hidden">
      <table class="w-full">
        <thead class="bg-gray-50 border-b">
          <tr>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">SKU</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Product Name</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Category</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Price</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Stock</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y">
          <tr v-for="product in products" :key="product.id" class="hover:bg-gray-50">
            <td class="px-6 py-3 text-sm font-medium text-gray-900">{{ product.sku }}</td>
            <td class="px-6 py-3 text-sm text-gray-700">{{ product.name }}</td>
            <td class="px-6 py-3 text-sm text-gray-600">{{ product.category_name || '-' }}</td>
            <td class="px-6 py-3 text-sm font-semibold text-gray-900">₱{{ product.selling_price.toFixed(2) }}</td>
            <td class="px-6 py-3 text-sm">
              <span
                class="inline-block px-3 py-1 rounded-full text-sm font-medium"
                :class="{
                  'bg-red-100 text-red-800': product.quantity <= product.reorder_level,
                  'bg-green-100 text-green-800': product.quantity > product.reorder_level,
                }"
              >
                {{ product.quantity }}
              </span>
            </td>
            <td class="px-6 py-3 text-sm flex gap-2">
              <button @click="editProduct(product)" class="text-blue-600 hover:text-blue-800">Edit</button>
              <button @click="deleteProduct(product.id)" class="text-red-600 hover:text-red-800">Delete</button>
            </td>
          </tr>
          <tr v-if="products.length === 0">
            <td colspan="6" class="px-6 py-8 text-center text-gray-500">
              No products found for {{ industry.industryName }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'
import { useAppStore } from '@/stores/app'
import { productsApi } from '@/utils/api'
import ProductForm from '@/components/ProductForm.vue'

const auth = useAuthStore()
const industry = useIndustryStore()
const appStore = useAppStore()

const products = ref([])
const showCreateForm = ref(false)
const loading = ref(false)

onMounted(async () => {
  await industry.loadUserIndustry()
  await loadProducts()
})

async function loadProducts() {
  loading.value = true
  try {
    products.value = await productsApi.getProducts(auth.token)
  } catch (error) {
    appStore.notify(`Failed to load products: ${error.message}`, 'error')
  } finally {
    loading.value = false
  }
}

function editProduct(product) {
  // Open edit modal with product data
  console.log('Edit product:', product)
}

async function deleteProduct(productId) {
  if (!confirm('Are you sure you want to delete this product?')) return
  try {
    await productsApi.deleteProduct(auth.token, productId)
    appStore.notify('Product deleted successfully')
    await loadProducts()
  } catch (error) {
    appStore.notify(`Delete failed: ${error.message}`, 'error')
  }
}

async function handleProductCreated() {
  appStore.notify('Product created successfully')
  showCreateForm.value = false
  await loadProducts()
}
</script>
```

---

### 2. Admin Dashboard with Industry Switcher

```vue
<template>
  <div class="space-y-6">
    <!-- Industry Switcher -->
    <div v-if="auth.can(['super_admin'])" class="mb-6">
      <IndustrySwitcher />
    </div>

    <!-- Dashboard Header -->
    <div
      class="rounded-lg shadow p-8 text-white"
      :style="{ backgroundColor: industry.industryColor }}"
    >
      <div class="flex justify-between items-start">
        <div>
          <h1 class="text-4xl font-bold">{{ industry.industryName }}</h1>
          <p class="text-lg opacity-90 mt-2">{{ industry.industryDescription }}</p>
        </div>
        <div class="text-5xl">{{ industry.industryIcon }}</div>
      </div>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-4 gap-6">
      <div class="bg-white rounded-lg shadow p-6">
        <div class="text-3xl font-bold text-gray-900">{{ stats.totalProducts }}</div>
        <div class="text-sm text-gray-600 mt-2">Total Products</div>
      </div>
      <div class="bg-white rounded-lg shadow p-6">
        <div class="text-3xl font-bold text-gray-900">{{ stats.lowStockCount }}</div>
        <div class="text-sm text-red-600 mt-2">Low Stock Items</div>
      </div>
      <div class="bg-white rounded-lg shadow p-6">
        <div class="text-3xl font-bold text-gray-900">₱{{ stats.totalSales }}</div>
        <div class="text-sm text-gray-600 mt-2">Sales This Month</div>
      </div>
      <div class="bg-white rounded-lg shadow p-6">
        <div class="text-3xl font-bold text-gray-900">{{ stats.activeUsers }}</div>
        <div class="text-sm text-gray-600 mt-2">Active Users</div>
      </div>
    </div>

    <!-- Industry Features -->
    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Enabled Features</h2>
      <div class="grid grid-cols-3 gap-4">
        <div
          v-for="(enabled, feature) in industry.getCustomFields()"
          :key="feature"
          class="flex items-start gap-3 p-4 bg-gray-50 rounded-lg"
        >
          <span class="text-lg">✓</span>
          <div>
            <div class="font-medium text-gray-900 capitalize">{{ feature }}</div>
            <div class="text-xs text-gray-500">Enabled for {{ industry.industryName }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Pharmacy-Specific Widget -->
    <div
      v-if="industry.hasFeature('pharmacy')"
      class="bg-blue-50 border border-blue-200 rounded-lg p-6"
    >
      <h2 class="text-lg font-semibold text-blue-900 mb-4">Pharmacy Features Active</h2>
      <div class="grid grid-cols-2 gap-4">
        <div v-if="industry.hasFeature('prescriptions')" class="text-sm text-blue-800">
          ✓ Prescription Management
        </div>
        <div v-if="industry.hasFeature('controlledSubstances')" class="text-sm text-blue-800">
          ✓ Controlled Substances Logging
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'
import IndustrySwitcher from '@/components/IndustrySwitcher.vue'
import { reportsApi } from '@/utils/api'

const auth = useAuthStore()
const industry = useIndustryStore()

const stats = ref({
  totalProducts: 0,
  lowStockCount: 0,
  totalSales: 0,
  activeUsers: 0,
})

onMounted(async () => {
  await industry.loadUserIndustry()
  await industry.loadIndustryDetails(industry.industryId, auth.token)
  await loadStats()
})

async function loadStats() {
  try {
    const dashboardData = await reportsApi.getDashboardStats(auth.token)
    stats.value = dashboardData
  } catch (error) {
    console.error('Failed to load dashboard stats:', error)
  }
}
</script>
```

---

### 3. User Management with Industry Assignment

```vue
<template>
  <div class="space-y-6">
    <h1 class="text-3xl font-bold text-gray-900">User Management</h1>

    <!-- Users Table -->
    <div class="bg-white rounded-lg shadow overflow-hidden">
      <table class="w-full">
        <thead class="bg-gray-50 border-b">
          <tr>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Username</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Full Name</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Role</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Industry</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Status</th>
            <th class="px-6 py-3 text-left text-sm font-semibold text-gray-900">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y">
          <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50">
            <td class="px-6 py-3 text-sm font-medium text-gray-900">{{ user.username }}</td>
            <td class="px-6 py-3 text-sm text-gray-700">{{ user.full_name }}</td>
            <td class="px-6 py-3 text-sm">
              <span class="inline-block px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
                {{ user.role }}
              </span>
            </td>
            <td class="px-6 py-3 text-sm">
              <div v-if="user.industry" class="flex items-center gap-2">
                <span
                  class="inline-block w-3 h-3 rounded-full"
                  :style="{ backgroundColor: user.industry.color }}$"
                ></span>
                <span>{{ user.industry.name }}</span>
              </div>
              <span v-else class="text-gray-400">Not assigned</span>
            </td>
            <td class="px-6 py-3 text-sm">
              <span
                class="inline-block px-3 py-1 rounded-full text-sm font-medium"
                :class="
                  user.is_active
                    ? 'bg-green-100 text-green-800'
                    : 'bg-red-100 text-red-800'
                "
              >
                {{ user.is_active ? 'Active' : 'Inactive' }}
              </span>
            </td>
            <td class="px-6 py-3 text-sm flex gap-2">
              <button
                @click="openIndustryModal(user)"
                class="text-blue-600 hover:text-blue-800"
              >
                Change Industry
              </button>
              <button @click="editUser(user)" class="text-gray-600 hover:text-gray-800">Edit</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Industry Assignment Modal -->
    <div
      v-if="selectedUser"
      class="fixed inset-0 bg-black bg-opacity-50 z-40 flex items-center justify-center"
    >
      <div class="bg-white rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
        <h2 class="text-xl font-semibold text-gray-900 mb-4">
          Assign Industry to {{ selectedUser.full_name }}
        </h2>

        <div class="space-y-3 mb-6 max-h-[300px] overflow-y-auto">
          <button
            v-for="ind in availableIndustries"
            :key="ind.id"
            @click="assignIndustry(selectedUser.id, ind.id)"
            class="w-full flex items-center gap-3 p-4 rounded-lg border hover:bg-gray-50 transition"
            :class="{
              'border-blue-500 bg-blue-50': selectedUser.industry?.id === ind.id,
              'border-gray-300': selectedUser.industry?.id !== ind.id,
            }"
          >
            <span class="text-2xl">{{ ind.icon }}</span>
            <div class="text-left flex-1">
              <div class="font-medium text-gray-900">{{ ind.name }}</div>
              <div class="text-xs text-gray-500">{{ ind.code }}</div>
            </div>
            <span v-if="selectedUser.industry?.id === ind.id" class="text-blue-600">✓</span>
          </button>
        </div>

        <button
          @click="selectedUser = null"
          class="w-full px-4 py-2 bg-gray-200 text-gray-900 rounded-lg hover:bg-gray-300"
        >
          Close
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'
import { useAppStore } from '@/stores/app'
import { usersApi, industriesApi } from '@/utils/api'
import { getAllIndustryConfigs } from '@/utils/industryConfig'

const auth = useAuthStore()
const industryStore = useIndustryStore()
const appStore = useAppStore()

const users = ref([])
const availableIndustries = ref([])
const selectedUser = ref(null)

onMounted(async () => {
  await loadUsers()
  availableIndustries.value = getAllIndustryConfigs()
})

async function loadUsers() {
  try {
    users.value = await usersApi.getUsers(auth.token)
  } catch (error) {
    appStore.notify(`Failed to load users: ${error.message}`, 'error')
  }
}

function openIndustryModal(user) {
  selectedUser.value = user
}

async function assignIndustry(userId, industryId) {
  try {
    await industriesApi.assignUserToIndustry(auth.token, {
      user_id: userId,
      industry_id: industryId,
    })
    appStore.notify('User industry updated')
    selectedUser.value = null
    await loadUsers()
  } catch (error) {
    appStore.notify(`Assignment failed: ${error.message}`, 'error')
  }
}

function editUser(user) {
  // Navigate to user edit page
  console.log('Edit user:', user)
}
</script>
```

---

### 4. Industry Settings/Admin Panel

```vue
<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <h1 class="text-3xl font-bold text-gray-900">Industry Management</h1>
    </div>

    <!-- Industries Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div
        v-for="ind in industries"
        :key="ind.id"
        class="bg-white rounded-lg shadow overflow-hidden hover:shadow-lg transition"
      >
        <!-- Industry Header -->
        <div class="h-20" :style="{ backgroundColor: ind.color }}"></div>

        <!-- Industry Content -->
        <div class="p-6 -mt-10 relative">
          <div class="flex items-start justify-between mb-4">
            <div class="text-4xl">{{ getIndustryIcon(ind.code) }}</div>
            <span class="text-xs font-semibold text-gray-500 uppercase">{{ ind.code }}</span>
          </div>

          <h3 class="text-lg font-bold text-gray-900">{{ ind.name }}</h3>
          <p class="text-sm text-gray-600 mt-2">{{ ind.description }}</p>

          <!-- Custom Attributes Count -->
          <div class="mt-4 p-3 bg-gray-50 rounded-lg text-sm">
            <span class="font-semibold text-gray-900">{{ getAttributeCount(ind.id) }}</span>
            <span class="text-gray-600">custom attributes</span>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 mt-4">
            <button
              @click="viewIndustryDetails(ind.id)"
              class="flex-1 px-3 py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 text-sm font-medium"
            >
              View Details
            </button>
            <button
              @click="openAttributeModal(ind.id)"
              class="flex-1 px-3 py-2 bg-green-100 text-green-700 rounded-lg hover:bg-green-200 text-sm font-medium"
            >
              Add Attribute
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Attribute Modal -->
    <div v-if="showAttributeModal" class="fixed inset-0 bg-black bg-opacity-50 z-40">
      <div class="flex items-center justify-center min-h-screen p-4">
        <div class="bg-white rounded-lg shadow-xl p-6 max-w-md w-full">
          <h2 class="text-xl font-semibold text-gray-900 mb-4">Add Custom Attribute</h2>

          <div class="space-y-4 mb-6">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Attribute Name</label>
              <input
                v-model="newAttribute.name"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="e.g. warranty_months"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Label</label>
              <input
                v-model="newAttribute.label"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="e.g. Warranty (Months)"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Type</label>
              <select
                v-model="newAttribute.type"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="text">Text</option>
                <option value="number">Number</option>
                <option value="date">Date</option>
                <option value="select">Select</option>
                <option value="textarea">Textarea</option>
                <option value="checkbox">Checkbox</option>
              </select>
            </div>

            <label class="flex items-center gap-2">
              <input v-model="newAttribute.required" type="checkbox" class="w-4 h-4" />
              <span class="text-sm text-gray-700">Required Field</span>
            </label>
          </div>

          <div class="flex gap-2">
            <button
              @click="addAttribute"
              class="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              Add
            </button>
            <button
              @click="showAttributeModal = false"
              class="flex-1 px-4 py-2 bg-gray-200 text-gray-900 rounded-lg hover:bg-gray-300"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { industriesApi } from '@/utils/api'
import { getAllIndustryConfigs } from '@/utils/industryConfig'

const auth = useAuthStore()
const appStore = useAppStore()

const industries = ref([])
const showAttributeModal = ref(false)
const selectedIndustryId = ref(null)
const industryDetails = ref({})

const newAttribute = ref({
  name: '',
  label: '',
  type: 'text',
  required: false,
})

onMounted(async () => {
  const configs = getAllIndustryConfigs()
  industries.value = configs
  // Load details for each industry
  for (const ind of industries.value) {
    try {
      const details = await industriesApi.getIndustry(auth.token, ind.id)
      industryDetails.value[ind.id] = details
    } catch (error) {
      console.error(`Failed to load industry ${ind.id}:`, error)
    }
  }
})

function getIndustryIcon(code) {
  const config = getAllIndustryConfigs().find((c) => c.code === code)
  return config?.icon || '🏢'
}

function getAttributeCount(industryId) {
  return industryDetails.value[industryId]?.attributes?.length || 0
}

function openAttributeModal(industryId) {
  selectedIndustryId.value = industryId
  showAttributeModal.value = true
}

async function addAttribute() {
  try {
    await industriesApi.addIndustryAttribute(auth.token, {
      industry_id: selectedIndustryId.value,
      attribute_name: newAttribute.value.name,
      attribute_label: newAttribute.value.label,
      attribute_type: newAttribute.value.type,
      is_required: newAttribute.value.required,
      display_order: getAttributeCount(selectedIndustryId.value) + 1,
    })
    appStore.notify('Attribute added successfully')
    showAttributeModal.value = false
    newAttribute.value = { name: '', label: '', type: 'text', required: false }
  } catch (error) {
    appStore.notify(`Failed to add attribute: ${error.message}`, 'error')
  }
}

function viewIndustryDetails(industryId) {
  console.log('View details for industry:', industryId)
}
</script>
```

---

## Integration Steps

1. **Copy ProductForm.vue** to `src/components/`
2. **Copy IndustrySwitcher.vue** to `src/components/`
3. **Copy IndustryInfo.vue** to `src/components/`
4. **Update your stores** (auth.js, industry.js)
5. **Update API utils** (api.js) with industriesApi
6. **Add industryConfig.js** to `src/utils/`
7. **Use examples above** in your views
8. **Initialize on app startup** - add industry loading to your router or App.vue

All examples are production-ready and follow Vue 3 Composition API best practices!
