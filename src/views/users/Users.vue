<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">User Management</h2>
      <button v-if="can(SUPER)" class="btn-primary" @click="openCreate">+ Add User</button>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead><tr><th>Name</th><th>Username</th><th>Email</th><th>Role</th><th>Status</th><th>Created</th><th v-if="can(SUPER)">Actions</th></tr></thead>
        <tbody>
          <tr v-if="!users.length"><td colspan="7" class="text-center py-8 text-gray-400">No users found</td></tr>
          <tr v-for="u in users" :key="u.id">
            <td class="font-medium">{{ u.full_name }}</td>
            <td class="font-mono text-sm">{{ u.username }}</td>
            <td class="text-gray-500">{{ u.email ?? '—' }}</td>
            <td><span class="badge" :class="roleColor(u.role)">{{ roleLabel(u.role) }}</span></td>
            <td><span class="badge" :class="u.is_active ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-500'">{{ u.is_active ? 'Active' : 'Inactive' }}</span></td>
            <td class="text-xs text-gray-400">{{ formatDate(u.created_at) }}</td>
            <td v-if="can(SUPER)">
              <div class="flex gap-1">
                <button class="btn btn-sm btn-secondary" @click="openEdit(u)">Edit</button>
                <button v-if="u.id !== authStore.user.id" class="btn btn-sm btn-danger" @click="confirmDelete(u)">{{ u.is_active ? 'Deactivate' : 'Activate' }}</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- My Account / Change Password -->
    <div class="card mt-6 max-w-md">
      <h3 class="text-sm font-semibold text-gray-700 mb-4">Change My Password</h3>
      <form @submit.prevent="changePassword" class="space-y-3">
        <div><label class="label">Current Password</label><input v-model="pwForm.current" type="password" class="input" /></div>
        <div><label class="label">New Password</label><input v-model="pwForm.newPw" type="password" class="input" /></div>
        <div><label class="label">Confirm New Password</label><input v-model="pwForm.confirm" type="password" class="input" /></div>
        <div v-if="pwError" class="text-sm text-red-600">{{ pwError }}</div>
        <button type="submit" class="btn-primary" :disabled="changingPw">{{ changingPw ? 'Saving…' : 'Change Password' }}</button>
      </form>
    </div>

    <Modal v-model="showModal" :title="editing ? `Edit ${editing.full_name}` : 'Add User'" width="500px">
      <form @submit.prevent="save" class="grid grid-cols-2 gap-4">
        <div class="col-span-2"><label class="label">Full Name *</label><input v-model="form.full_name" class="input" required /></div>
        <div><label class="label">Username *</label><input v-model="form.username" class="input" required :readonly="!!editing" :class="editing ? 'bg-gray-50' : ''" /></div>
        <div><label class="label">Email</label><input v-model="form.email" type="email" class="input" /></div>
        <div>
          <label class="label">Role *</label>
          <select v-model="form.role" class="input" required>
            <option v-for="r in ROLES" :key="r.value" :value="r.value">{{ r.label }}</option>
          </select>
        </div>
        <div v-if="!editing"><label class="label">Password *</label><input v-model="form.password" type="password" class="input" required minlength="6" /></div>
        <div v-if="editing"><label class="label">New Password (leave blank to keep)</label><input v-model="form.password" type="password" class="input" minlength="6" /></div>
        <div v-if="editing" class="flex items-center gap-2 col-span-2">
          <input type="checkbox" v-model="form.is_active" id="is_active" class="w-4 h-4" />
          <label for="is_active" class="text-sm text-gray-700">Account Active</label>
        </div>
        <div v-if="formError" class="col-span-2 text-sm text-red-600">{{ formError }}</div>
        <div class="col-span-2 flex justify-end gap-3">
          <button type="button" class="btn-secondary" @click="showModal = false">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save User' }}</button>
        </div>
      </form>
    </Modal>

    <ConfirmDialog
      v-model="showDeleteDialog"
      title="Deactivate User"
      :message="`Deactivate '${deletingUser?.full_name}'? They will no longer be able to login.`"
      confirm-label="Deactivate"
      :danger="true"
      @confirm="doDelete"
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { usersApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatDate, roleLabel, roleColor, ROLES, ACCESS } from '@/utils/format'
import Modal from '@/components/common/Modal.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'

const authStore = useAuthStore()
const appStore = useAppStore()
const { SUPER } = ACCESS
const can = (r) => authStore.can(r)

const users = ref([])
const showModal = ref(false)
const editing = ref(null)
const saving = ref(false)
const formError = ref('')
const showDeleteDialog = ref(false)
const deletingUser = ref(null)
const form = ref({ username: '', full_name: '', email: '', role: 'cashier', password: '', is_active: true })
const pwForm = ref({ current: '', newPw: '', confirm: '' })
const pwError = ref('')
const changingPw = ref(false)

async function load() {
  try { users.value = await usersApi.getUsers(authStore.token) }
  catch (e) { appStore.notify(e.message, 'error') }
}

function openCreate() { editing.value = null; form.value = { username: '', full_name: '', email: '', role: 'cashier', password: '', is_active: true }; formError.value = ''; showModal.value = true }
function openEdit(u) { editing.value = u; form.value = { ...u, password: '' }; formError.value = ''; showModal.value = true }
function confirmDelete(u) { deletingUser.value = u; showDeleteDialog.value = true }

async function save() {
  saving.value = true; formError.value = ''
  try {
    if (editing.value) {
      await usersApi.updateUser(authStore.token, editing.value.id, {
        full_name: form.value.full_name,
        email: form.value.email || null,
        role: form.value.role,
        is_active: form.value.is_active,
        password: form.value.password || null,
      })
      appStore.notify('User updated')
    } else {
      await usersApi.createUser(authStore.token, form.value)
      appStore.notify('User created')
    }
    showModal.value = false; load()
  } catch (e) { formError.value = e.message }
  finally { saving.value = false }
}

async function doDelete() {
  try { await usersApi.deleteUser(authStore.token, deletingUser.value.id); appStore.notify('User deactivated'); load() }
  catch (e) { appStore.notify(e.message, 'error') }
}

async function changePassword() {
  pwError.value = ''
  if (pwForm.value.newPw !== pwForm.value.confirm) { pwError.value = 'Passwords do not match'; return }
  if (pwForm.value.newPw.length < 6) { pwError.value = 'Password must be at least 6 characters'; return }
  changingPw.value = true
  try {
    await usersApi.changePassword(authStore.token, pwForm.value.current, pwForm.value.newPw)
    appStore.notify('Password changed successfully')
    pwForm.value = { current: '', newPw: '', confirm: '' }
  } catch (e) { pwError.value = e.message }
  finally { changingPw.value = false }
}

onMounted(load)
</script>
