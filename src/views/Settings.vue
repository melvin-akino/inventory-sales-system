<template>
  <div class="max-w-2xl">
    <div class="page-header">
      <h2 class="page-title">System Settings</h2>
    </div>

    <form @submit.prevent="save" class="space-y-6">
      <!-- Company Info -->
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-700 mb-4">Company Information</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="col-span-2">
            <label class="label">Company Name</label>
            <input v-model="settings.company_name" class="input" />
          </div>
          <div class="col-span-2">
            <label class="label">Address</label>
            <input v-model="settings.company_address" class="input" />
          </div>
          <div>
            <label class="label">Phone</label>
            <input v-model="settings.company_phone" class="input" />
          </div>
          <div>
            <label class="label">Email</label>
            <input v-model="settings.company_email" type="email" class="input" />
          </div>
          <div>
            <label class="label">TIN Number</label>
            <input v-model="settings.company_tin" class="input" placeholder="000-000-000-000" />
          </div>
        </div>
      </div>

      <!-- Tax & Numbering -->
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-700 mb-4">Tax & Numbering</h3>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="label">VAT Rate (%)</label>
            <input v-model="settings.vat_rate" type="number" min="0" max="100" class="input" />
          </div>
          <div>
            <label class="label">Currency Symbol</label>
            <input v-model="settings.currency_symbol" class="input" />
          </div>
          <div>
            <label class="label">Invoice/OR Prefix</label>
            <input v-model="settings.invoice_prefix" class="input" placeholder="OR" />
          </div>
          <div>
            <label class="label">Sale Number Prefix</label>
            <input v-model="settings.sale_prefix" class="input" placeholder="SL" />
          </div>
          <div>
            <label class="label">Low Stock Threshold</label>
            <input v-model="settings.low_stock_threshold" type="number" min="0" class="input" />
          </div>
        </div>
      </div>

      <!-- Receipt -->
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-700 mb-4">Receipt</h3>
        <div>
          <label class="label">Receipt Footer Message</label>
          <input v-model="settings.receipt_footer" class="input" />
        </div>
      </div>

      <div v-if="error" class="text-sm text-red-600">{{ error }}</div>

      <button type="submit" class="btn-primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save Settings' }}</button>
    </form>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { settingsApi } from '@/utils/api'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'

const authStore = useAuthStore()
const appStore = useAppStore()
const saving = ref(false)
const error = ref('')

const settings = ref({
  company_name: '',
  company_address: '',
  company_phone: '',
  company_email: '',
  company_tin: '',
  currency_symbol: '₱',
  vat_rate: '12',
  invoice_prefix: 'OR',
  sale_prefix: 'SL',
  low_stock_threshold: '10',
  receipt_footer: 'Thank you for your business!',
})

async function load() {
  try {
    const s = await settingsApi.getSettings(authStore.token)
    Object.assign(settings.value, s)
    appStore.settings = s
  } catch (e) { appStore.notify(e.message, 'error') }
}

async function save() {
  saving.value = true; error.value = ''
  try {
    await settingsApi.updateSettings(authStore.token, settings.value)
    appStore.settings = { ...settings.value }
    appStore.notify('Settings saved successfully')
  } catch (e) { error.value = e.message }
  finally { saving.value = false }
}

onMounted(load)
</script>
