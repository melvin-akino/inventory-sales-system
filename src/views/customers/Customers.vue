<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">Customers</h2>
      <button v-if="can(SALES)" class="btn-primary" @click="openCreate">+ Add Customer</button>
    </div>

    <div class="card mb-4">
      <input v-model="search" class="input w-72" placeholder="🔍 Search name, phone, or TIN…" @input="load" />
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead><tr><th>Name</th><th>Phone</th><th>Email</th><th>TIN</th><th>Address</th><th v-if="can(MANAGE)">Actions</th></tr></thead>
        <tbody>
          <tr v-if="!customers.length"><td colspan="6" class="text-center py-8 text-gray-400">No customers found</td></tr>
          <tr v-for="c in customers" :key="c.id">
            <td class="font-medium">{{ c.name }}</td>
            <td class="text-gray-500">{{ c.phone ?? '—' }}</td>
            <td class="text-gray-500">{{ c.email ?? '—' }}</td>
            <td class="font-mono text-xs">{{ c.tin_number ?? '—' }}</td>
            <td class="text-gray-500 max-w-xs truncate">{{ c.address ?? '—' }}</td>
            <td v-if="can(MANAGE)">
              <div class="flex gap-1">
                <button class="btn btn-sm btn-secondary" @click="openEdit(c)">Edit</button>
                <button class="btn btn-sm btn-danger" @click="confirmDelete(c)">Delete</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <Modal v-model="showModal" :title="editing ? 'Edit Customer' : 'Add Customer'" width="500px">
      <form @submit.prevent="save" class="grid grid-cols-2 gap-4">
        <div class="col-span-2">
          <label class="label">Full Name *</label>
          <input v-model="form.name" class="input" required />
        </div>
        <div><label class="label">Phone</label><input v-model="form.phone" class="input" /></div>
        <div><label class="label">Email</label><input v-model="form.email" type="email" class="input" /></div>
        <div><label class="label">TIN Number</label><input v-model="form.tin_number" class="input" placeholder="000-000-000-000" /></div>
        <div class="col-span-2"><label class="label">Address</label><input v-model="form.address" class="input" /></div>
        <div v-if="error" class="col-span-2 text-sm text-red-600">{{ error }}</div>
        <div class="col-span-2 flex justify-end gap-3">
          <button type="button" class="btn-secondary" @click="showModal = false">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save' }}</button>
        </div>
      </form>
    </Modal>

    <ConfirmDialog v-model="showDelete" title="Delete Customer" :message="`Remove '${deletingCustomer?.name}'?`" confirm-label="Delete" :danger="true" @confirm="deleteCustomer" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { customersApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { ACCESS } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'

const auth = useAuthStore()
const appStore = useAppStore()
const { MANAGE, SALES } = ACCESS
const can = (r) => auth.can(r)
const customers = ref([])
const search = ref('')
const showModal = ref(false)
const saving = ref(false)
const error = ref('')
const editing = ref(null)
const showDelete = ref(false)
const deletingCustomer = ref(null)
const form = ref({ name: '', phone: '', email: '', address: '', tin_number: '' })

async function load() {
  try { customers.value = await customersApi.getCustomers(auth.token, search.value || null) }
  catch (e) { appStore.notify(e.message, 'error') }
}

function openCreate() { editing.value = null; form.value = { name: '', phone: '', email: '', address: '', tin_number: '' }; error.value = ''; showModal.value = true }
function openEdit(c) { editing.value = c; form.value = { ...c }; error.value = ''; showModal.value = true }
function confirmDelete(c) { deletingCustomer.value = c; showDelete.value = true }

async function save() {
  saving.value = true; error.value = ''
  try {
    if (editing.value) { await customersApi.updateCustomer(auth.token, editing.value.id, form.value); appStore.notify('Customer updated') }
    else { await customersApi.createCustomer(auth.token, form.value); appStore.notify('Customer created') }
    showModal.value = false; load()
  } catch (e) { error.value = e.message }
  finally { saving.value = false }
}

async function deleteCustomer() {
  try { await customersApi.deleteCustomer(auth.token, deletingCustomer.value.id); appStore.notify('Customer removed'); load() }
  catch (e) { appStore.notify(e.message, 'error') }
}

onMounted(load)
</script>
