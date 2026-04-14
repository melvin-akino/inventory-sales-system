import { createRouter, createWebHashHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { ACCESS, canAccess } from '@/utils/format'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue'),
    meta: { public: true },
  },
  {
    path: '/help',
    name: 'Help',
    component: () => import('@/views/Help.vue'),
    meta: { public: true },
  },
  {
    path: '/',
    component: () => import('@/components/layout/AppLayout.vue'),
    meta: { requiresAuth: true },
    children: [
      {
        path: '',
        redirect: '/dashboard',
      },
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('@/views/Dashboard.vue'),
        meta: { title: 'Dashboard', roles: ACCESS.FULL },
      },
      // Inventory
      {
        path: 'inventory/products',
        name: 'Products',
        component: () => import('@/views/inventory/Products.vue'),
        meta: { title: 'Products', roles: ACCESS.FULL },
      },
      {
        path: 'inventory/categories',
        name: 'Categories',
        component: () => import('@/views/inventory/Categories.vue'),
        meta: { title: 'Categories', roles: ACCESS.MANAGE },
      },
      {
        path: 'inventory/stock-adjustments',
        name: 'StockAdjustments',
        component: () => import('@/views/inventory/StockAdjustments.vue'),
        meta: { title: 'Stock Adjustments', roles: ACCESS.MANAGE },
      },
      // Sales
      {
        path: 'sales/pos',
        name: 'POS',
        component: () => import('@/views/sales/POS.vue'),
        meta: { title: 'Point of Sale', roles: ACCESS.SALES },
      },
      {
        path: 'sales/history',
        name: 'SalesHistory',
        component: () => import('@/views/sales/SalesHistory.vue'),
        meta: { title: 'Sales History', roles: ACCESS.SALES },
      },
      // Invoices
      {
        path: 'invoices',
        name: 'Invoices',
        component: () => import('@/views/invoices/Invoices.vue'),
        meta: { title: 'Invoices / OR', roles: ACCESS.SALES },
      },
      // Customers
      {
        path: 'customers',
        name: 'Customers',
        component: () => import('@/views/customers/Customers.vue'),
        meta: { title: 'Customers', roles: ACCESS.FULL },
      },
      // Suppliers
      {
        path: 'suppliers',
        name: 'Suppliers',
        component: () => import('@/views/suppliers/Suppliers.vue'),
        meta: { title: 'Suppliers', roles: ACCESS.MANAGE },
      },
      // Reports
      {
        path: 'reports/sales',
        name: 'SalesReport',
        component: () => import('@/views/reports/SalesReport.vue'),
        meta: { title: 'Sales Report', roles: ACCESS.MANAGE },
      },
      {
        path: 'reports/inventory',
        name: 'InventoryReport',
        component: () => import('@/views/reports/InventoryReport.vue'),
        meta: { title: 'Inventory Report', roles: ACCESS.MANAGE },
      },
      {
        path: 'reports/profit-loss',
        name: 'ProfitLoss',
        component: () => import('@/views/reports/ProfitLoss.vue'),
        meta: { title: 'Profit & Loss', roles: ACCESS.ADMIN },
      },
      {
        path: 'reports/vat',
        name: 'VatReport',
        component: () => import('@/views/reports/VatReport.vue'),
        meta: { title: 'VAT Report (BIR)', roles: ACCESS.ADMIN },
      },
      // Users
      {
        path: 'users',
        name: 'Users',
        component: () => import('@/views/users/Users.vue'),
        meta: { title: 'User Management', roles: ACCESS.ADMIN },
      },
      // Settings
      {
        path: 'settings',
        name: 'Settings',
        component: () => import('@/views/Settings.vue'),
        meta: { title: 'System Settings', roles: ACCESS.SUPER },
      },
    ],
  },
  { path: '/:pathMatch(.*)*', redirect: '/dashboard' },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior() {
    return { top: 0 }
  },
})

router.beforeEach(async (to, from, next) => {
  const auth = useAuthStore()

  if (to.meta.public) return next()

  if (!auth.isAuthenticated) {
    const restored = await auth.restoreSession()
    if (!restored) return next('/login')
  }

  if (to.meta.roles && !canAccess(auth.role, to.meta.roles)) {
    return next('/dashboard')
  }

  next()
})

export default router
