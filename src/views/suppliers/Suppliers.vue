<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Suppliers</h2>
      <button class="btn-primary" @click="openCreate">+ Add Supplier</button>
    </div>
    <div class="card mb-4">
      <input v-model="search" class="input w-72" placeholder="🔍 Search supplier name…" @input="load" />
    </div>
    <div class="table-container">
      <table class="data-table">
        <thead><tr><th>Name</th><th>Contact Person</th><th>Phone</th><th>Email</th><th>TIN</th><th>Actions</th></tr></thead>
        <tbody>
          <tr v-if="!suppliers.length"><td colspan="6" class="text-center py-8 text-gray-400">No suppliers found</td></tr>
          <tr v-for="s in suppliers" :key="s.id">
            <td class="font-medium">{{ s.name }}</td>
            <td>{{ s.contact_person ?? '—' }}</td>
            <td class="text-gray-500">{{ s.phone ?? '—' }}</td>
            <td class="text-gray-500">{{ s.email ?? '—' }}</td>
            <td class="font-mono text-xs">{{ s.tin_number ?? '—' }}</td>
            <td>
              <div class="flex gap-1">
                <button class="btn btn-sm btn-secondary" @click="openEdit(s)">Edit</button>
                <button class="btn btn-sm btn-danger" @click="confirmDelete(s)">Delete</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <Modal v-model="showModal" :title="editing ? 'Edit Supplier' : 'Add Supplier'" width="500px">
      <form @submit.prevent="save" class="grid grid-cols-2 gap-4">
        <div class="col-span-2"><label class="label">Company Name *</label><input v-model="form.name" class="input" required /></div>
        <div><label class="label">Contact Person</label><input v-model="form.contact_person" class="input" /></div>
        <div><label class="label">Phone</label><input v-model="form.phone" class="input" /></div>
        <div><label class="label">Email</label><input v-model="form.email" type="email" class="input" /></div>
        <div><label class="label">TIN</label><input v-model="form.tin_number" class="input" /></div>
        <div class="col-span-2"><label class="label">Address</label><input v-model="form.address" class="input" /></div>
        <div v-if="error" class="col-span-2 text-sm text-red-600">{{ error }}</div>
        <div class="col-span-2 flex justify-end gap-3">
          <button type="button" class="btn-secondary" @click="showModal = false">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save' }}</button>
        </div>
      </form>
    </Modal>

    <ConfirmDialog v-model="showDelete" title="Delete Supplier" :message="`Remove '${deletingItem?.name}'?`" confirm-label="Delete" :danger="true" @confirm="doDelete" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { suppliersApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import Modal from '@/components/common/Modal.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'

const auth = useAuthStore()
const appStore = useAppStore()
const suppliers = ref([])
const search = ref('')
const showModal = ref(false)
const saving = ref(false)
const error = ref('')
const editing = ref(null)
const showDelete = ref(false)
const deletingItem = ref(null)
const form = ref({ name: '', contact_person: '', phone: '', email: '', address: '', tin_number: '' })

async function load() {
  try { suppliers.value = await suppliersApi.getSuppliers(auth.token, search.value || null) }
  catch (e) { appStore.notify(e.message, 'error') }
}

function openCreate() { editing.value = null; form.value = { name: '', contact_person: '', phone: '', email: '', address: '', tin_number: '' }; error.value = ''; showModal.value = true }
function openEdit(s) { editing.value = s; form.value = { ...s }; error.value = ''; showModal.value = true }
function confirmDelete(s) { deletingItem.value = s; showDelete.value = true }

async function save() {
  saving.value = true; error.value = ''
  try {
    if (editing.value) { await suppliersApi.updateSupplier(auth.token, editing.value.id, form.value); appStore.notify('Supplier updated') }
    else { await suppliersApi.createSupplier(auth.token, form.value); appStore.notify('Supplier created') }
    showModal.value = false; load()
  } catch (e) { error.value = e.message }
  finally { saving.value = false }
}

async function doDelete() {
  try { await suppliersApi.deleteSupplier(auth.token, deletingItem.value.id); appStore.notify('Supplier removed'); load() }
  catch (e) { appStore.notify(e.message, 'error') }
}

onMounted(load)
</script>
