<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Categories</h2>
      <button class="btn-primary" @click="openCreate">+ Add Category</button>
    </div>
    <div class="table-container">
      <table class="data-table">
        <thead><tr><th>Name</th><th>Description</th><th>Created</th><th>Actions</th></tr></thead>
        <tbody>
          <tr v-if="!categories.length"><td colspan="4" class="text-center py-8 text-gray-400">No categories found</td></tr>
          <tr v-for="c in categories" :key="c.id">
            <td class="font-medium">{{ c.name }}</td>
            <td class="text-gray-500">{{ c.description ?? '—' }}</td>
            <td class="text-gray-400 text-xs">{{ formatDate(c.created_at) }}</td>
            <td><button class="btn btn-sm btn-secondary" @click="openEdit(c)">Edit</button></td>
          </tr>
        </tbody>
      </table>
    </div>

    <Modal v-model="showModal" :title="editing ? 'Edit Category' : 'Add Category'" width="420px">
      <form @submit.prevent="save" class="space-y-4">
        <div>
          <label class="label">Category Name *</label>
          <input v-model="form.name" class="input" required placeholder="e.g. LED Bulbs" />
        </div>
        <div>
          <label class="label">Description</label>
          <input v-model="form.description" class="input" placeholder="Optional description" />
        </div>
        <div v-if="error" class="text-sm text-red-600">{{ error }}</div>
        <div class="flex justify-end gap-3">
          <button type="button" class="btn-secondary" @click="showModal = false">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save' }}</button>
        </div>
      </form>
    </Modal>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { categoriesApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatDate } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'

const auth = useAuthStore()
const appStore = useAppStore()
const categories = ref([])
const showModal = ref(false)
const saving = ref(false)
const error = ref('')
const editing = ref(null)
const form = ref({ name: '', description: '' })

async function load() {
  try { categories.value = await categoriesApi.getCategories(auth.token) } catch (e) { appStore.notify(e.message, 'error') }
}

function openCreate() { editing.value = null; form.value = { name: '', description: '' }; error.value = ''; showModal.value = true }
function openEdit(c) { editing.value = c; form.value = { name: c.name, description: c.description ?? '' }; error.value = ''; showModal.value = true }

async function save() {
  saving.value = true; error.value = ''
  try {
    if (editing.value) {
      await categoriesApi.updateCategory(auth.token, editing.value.id, form.value)
      appStore.notify('Category updated')
    } else {
      await categoriesApi.createCategory(auth.token, form.value)
      appStore.notify('Category created')
    }
    showModal.value = false; load()
  } catch (e) { error.value = e.message }
  finally { saving.value = false }
}

onMounted(load)
</script>
