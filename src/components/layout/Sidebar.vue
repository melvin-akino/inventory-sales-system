<template>
  <aside
    class="flex-shrink-0 bg-gray-900 text-gray-100 flex flex-col transition-all duration-300"
    :class="appStore.sidebarOpen ? 'w-60' : 'w-16'"
  >
    <!-- Logo -->
    <div class="flex items-center gap-3 px-4 py-5 border-b border-gray-700">
      <div class="w-8 h-8 rounded-lg bg-primary-500 flex items-center justify-center flex-shrink-0 text-white font-bold text-sm">L</div>
      <span v-if="appStore.sidebarOpen" class="font-bold text-white text-sm leading-tight">
        {{ appStore.getSetting('company_name', 'LumiSync') }}<br>
        <span class="text-xs font-normal text-gray-400">Inventory System</span>
      </span>
    </div>

    <!-- Nav -->
    <nav class="flex-1 overflow-y-auto py-4 space-y-0.5">
      <NavItem to="/dashboard" icon="📊" label="Dashboard" />

      <NavSection v-if="appStore.sidebarOpen" label="Inventory" />
      <NavItem to="/inventory/products" icon="📦" label="Products" />
      <NavItem v-if="can(MANAGE)" to="/inventory/categories" icon="🗂️" label="Categories" />
      <NavItem v-if="can(MANAGE)" to="/inventory/stock-adjustments" icon="⚖️" label="Stock Adjustments" />

      <NavSection v-if="appStore.sidebarOpen" label="Sales" />
      <NavItem v-if="can(SALES)" to="/sales/pos" icon="🛒" label="Point of Sale" />
      <NavItem v-if="can(SALES)" to="/sales/history" icon="📋" label="Sales History" />
      <NavItem v-if="can(SALES)" to="/invoices" icon="🧾" label="Invoices / OR" />

      <NavSection v-if="appStore.sidebarOpen" label="CRM" />
      <NavItem to="/customers" icon="👥" label="Customers" />
      <NavItem v-if="can(MANAGE)" to="/suppliers" icon="🏭" label="Suppliers" />

      <NavSection v-if="appStore.sidebarOpen && can(MANAGE)" label="Reports" />
      <NavItem v-if="can(MANAGE)" to="/reports/sales" icon="📈" label="Sales Report" />
      <NavItem v-if="can(MANAGE)" to="/reports/inventory" icon="📉" label="Inventory Report" />
      <NavItem v-if="can(ADMIN)" to="/reports/profit-loss" icon="💹" label="Profit & Loss" />
      <NavItem v-if="can(ADMIN)" to="/reports/vat" icon="🏛️" label="VAT Report (BIR)" />

      <NavSection v-if="appStore.sidebarOpen && can(ADMIN)" label="Administration" />
      <NavItem v-if="can(ADMIN)" to="/users" icon="👤" label="User Management" />
      <NavItem v-if="can(SUPER)" to="/settings" icon="⚙️" label="Settings" />

      <div class="border-t border-gray-700 mt-2 pt-2">
        <NavItem to="/help" icon="❓" label="Help Center" />
      </div>
    </nav>

    <!-- User footer -->
    <div class="border-t border-gray-700 p-4">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-full bg-primary-600 flex items-center justify-center text-white text-xs font-bold flex-shrink-0">
          {{ authStore.user?.full_name?.[0] ?? 'U' }}
        </div>
        <div v-if="appStore.sidebarOpen" class="flex-1 min-w-0">
          <p class="text-xs font-medium text-white truncate">{{ authStore.user?.full_name }}</p>
          <p class="text-xs text-gray-400 capitalize">{{ authStore.user?.role?.replace('_', ' ') }}</p>
        </div>
        <button
          v-if="appStore.sidebarOpen"
          @click="logout"
          class="text-gray-400 hover:text-white text-xs flex-shrink-0"
          title="Logout"
        >⏻</button>
      </div>
    </div>
  </aside>
</template>

<script setup>
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { useRouter } from 'vue-router'
import { ACCESS } from '@/utils/format'
import NavItem from './NavItem.vue'
import NavSection from './NavSection.vue'

const authStore = useAuthStore()
const appStore = useAppStore()
const router = useRouter()

const { MANAGE, ADMIN, SUPER, SALES } = ACCESS
const can = (roles) => authStore.can(roles)

async function logout() {
  await authStore.logout()
  router.push('/login')
}
</script>
