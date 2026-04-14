<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200 shadow-sm sticky top-0 z-40">
      <div class="max-w-4xl mx-auto px-6 py-4 flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Help & Support</h1>
          <p class="text-sm text-gray-600">{{ companyName }} System Guide</p>
        </div>
        <router-link 
          to="/login" 
          class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
        >
          ← Back to Login
        </router-link>
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-4xl mx-auto px-6 py-8">
      <!-- Getting Started -->
      <section class="bg-white rounded-lg shadow-sm p-8 mb-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">🚀 Getting Started</h2>
        <div class="space-y-4 text-gray-700">
          <p><strong>Default Login Credentials:</strong></p>
          <ul class="list-disc list-inside space-y-2 ml-2">
            <li><strong>Username:</strong> admin</li>
            <li><strong>Password:</strong> Admin@123</li>
          </ul>
          <p class="mt-4 text-sm text-gray-600">
            Use these credentials to sign in to the system. After logging in, you can update your password in Settings.
          </p>
        </div>
      </section>

      <!-- Main Features -->
      <section class="bg-white rounded-lg shadow-sm p-8 mb-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">📋 Main Features</h2>
        <div class="grid md:grid-cols-2 gap-6">
          <!-- Dashboard -->
          <div class="border border-gray-200 rounded-lg p-4">
            <h3 class="font-semibold text-gray-900 mb-2">📊 Dashboard</h3>
            <p class="text-sm text-gray-600">
              View sales summary, inventory status, and key metrics at a glance. Access reports and analytics.
            </p>
          </div>

          <!-- POS -->
          <div class="border border-gray-200 rounded-lg p-4">
            <h3 class="font-semibold text-gray-900 mb-2">🛒 Point of Sale (POS)</h3>
            <p class="text-sm text-gray-600">
              Quick checkout interface. Browse products by category, add items to cart, apply discounts, and complete sales.
            </p>
          </div>

          <!-- Inventory -->
          <div class="border border-gray-200 rounded-lg p-4">
            <h3 class="font-semibold text-gray-900 mb-2">📦 Inventory</h3>
            <p class="text-sm text-gray-600">
              Manage products, categories, stock levels, and reorder points. Track inventory movements and adjustments.
            </p>
          </div>

          <!-- Sales -->
          <div class="border border-gray-200 rounded-lg p-4">
            <h3 class="font-semibold text-gray-900 mb-2">💰 Sales</h3>
            <p class="text-sm text-gray-600">
              View sales history, invoices, and transaction details. Generate receipts and manage returns.
            </p>
          </div>

          <!-- Customers -->
          <div class="border border-gray-200 rounded-lg p-4">
            <h3 class="font-semibold text-gray-900 mb-2">👥 Customers</h3>
            <p class="text-sm text-gray-600">
              Manage customer information, purchase history, and contact details. Track customer interactions.
            </p>
          </div>

          <!-- Settings -->
          <div class="border border-gray-200 rounded-lg p-4">
            <h3 class="font-semibold text-gray-900 mb-2">⚙️ Settings</h3>
            <p class="text-sm text-gray-600">
              Configure company information, users, tax rates, and system preferences.
            </p>
          </div>
        </div>
      </section>

      <!-- How to Use POS -->
      <section class="bg-white rounded-lg shadow-sm p-8 mb-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">🛒 How to Use Point of Sale</h2>
        <ol class="space-y-3 text-gray-700">
          <li>
            <strong>1. Navigate to POS:</strong>
            Click "Sales" → "Point of Sale" from the main menu.
          </li>
          <li>
            <strong>2. Browse Products:</strong>
            See the product grid on the left. Click category buttons to filter by category.
          </li>
          <li>
            <strong>3. Search Products:</strong>
            Use the search bar to find products by name or SKU.
          </li>
          <li>
            <strong>4. Add to Cart:</strong>
            Click any product to add it to the cart. Click again to add more units.
          </li>
          <li>
            <strong>5. Manage Cart:</strong>
            Use [−] and [+] buttons to adjust quantities. Click [✕] to remove items.
          </li>
          <li>
            <strong>6. Apply Discounts:</strong>
            Enter discount percentage or fixed amount. Totals update automatically.
          </li>
          <li>
            <strong>7. Select Payment Method:</strong>
            Choose from Cash, Card, Check, or Mixed payment options.
          </li>
          <li>
            <strong>8. Enter Amount Paid:</strong>
            Type the amount customer provided. Change is calculated automatically.
          </li>
          <li>
            <strong>9. Complete Sale:</strong>
            Click "Complete Sale" button to finalize the transaction.
          </li>
          <li>
            <strong>10. Print Receipt:</strong>
            Optionally print the receipt for the customer.
          </li>
        </ol>
      </section>

      <!-- Managing Inventory -->
      <section class="bg-white rounded-lg shadow-sm p-8 mb-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">📦 Managing Inventory</h2>
        <div class="space-y-3 text-gray-700">
          <p>
            <strong>Add Products:</strong>
            Go to Inventory → Products → Add Product. Fill in product details including name, SKU, category, cost price, selling price, and quantity.
          </p>
          <p>
            <strong>Edit Products:</strong>
            Click on a product to edit its information. Changes are saved immediately.
          </p>
          <p>
            <strong>Manage Categories:</strong>
            Create and organize product categories from Inventory → Categories.
          </p>
          <p>
            <strong>Stock Adjustments:</strong>
            Adjust inventory levels when items arrive, are damaged, or need correction.
          </p>
          <p>
            <strong>Low Stock Alerts:</strong>
            Items below the reorder level are highlighted, reminding you when to restock.
          </p>
        </div>
      </section>

      <!-- Company Settings -->
      <section class="bg-white rounded-lg shadow-sm p-8 mb-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">⚙️ Company Settings</h2>
        <div class="space-y-3 text-gray-700">
          <p>
            <strong>Update Company Information:</strong>
            Go to Settings → Company. Update your company name, address, phone, email, and tax number.
          </p>
          <p>
            <strong>Configure Tax Rates:</strong>
            Set VAT rate and other tax configurations for automatic calculation on sales.
          </p>
          <p>
            <strong>User Management:</strong>
            Create and manage user accounts with different roles and permissions.
          </p>
          <p>
            <strong>Change Password:</strong>
            Update your login password from Settings → Profile.
          </p>
        </div>
      </section>

      <!-- Troubleshooting -->
      <section class="bg-blue-50 border border-blue-200 rounded-lg p-8 mb-6">
        <h2 class="text-xl font-bold text-blue-900 mb-4">🔧 Troubleshooting</h2>
        <div class="space-y-4 text-blue-800">
          <div>
            <p><strong>Can't log in?</strong></p>
            <p class="text-sm">Check that you're using the correct username and password. Default is admin / Admin@123. Contact your administrator if you need a password reset.</p>
          </div>
          <div>
            <p><strong>Products not showing in POS?</strong></p>
            <p class="text-sm">Make sure products have been created in Inventory → Products. Check that products are marked as active and have a selling price set.</p>
          </div>
          <div>
            <p><strong>Sales not saving?</strong></p>
            <p class="text-sm">Verify that the backend server is running. Check your internet connection if using network mode.</p>
          </div>
          <div>
            <p><strong>Discounts not working?</strong></p>
            <p class="text-sm">Enter discount as percentage (0-100) or fixed amount in PHP currency. Totals should update automatically after entering discount.</p>
          </div>
          <div>
            <p><strong>Need to clear browser cache?</strong></p>
            <p class="text-sm">Press Ctrl+Shift+R (Windows/Linux) or Cmd+Shift+R (Mac) to force refresh and clear cache.</p>
          </div>
        </div>
      </section>

      <!-- Tips & Best Practices -->
      <section class="bg-white rounded-lg shadow-sm p-8 mb-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">💡 Tips & Best Practices</h2>
        <ul class="space-y-2 text-gray-700">
          <li class="flex items-start">
            <span class="text-primary-500 mr-3 font-bold">✓</span>
            <span>Use unique SKUs for each product to avoid confusion.</span>
          </li>
          <li class="flex items-start">
            <span class="text-primary-500 mr-3 font-bold">✓</span>
            <span>Keep cost prices updated for accurate profit calculations.</span>
          </li>
          <li class="flex items-start">
            <span class="text-primary-500 mr-3 font-bold">✓</span>
            <span>Set appropriate reorder levels to avoid stockouts.</span>
          </li>
          <li class="flex items-start">
            <span class="text-primary-500 mr-3 font-bold">✓</span>
            <span>Review daily sales reports to track business performance.</span>
          </li>
          <li class="flex items-start">
            <span class="text-primary-500 mr-3 font-bold">✓</span>
            <span>Organize products into categories for faster browsing in POS.</span>
          </li>
          <li class="flex items-start">
            <span class="text-primary-500 mr-3 font-bold">✓</span>
            <span>Create customer records for loyalty tracking and history.</span>
          </li>
        </ul>
      </section>

      <!-- Contact Support -->
      <section class="bg-gradient-to-r from-primary-50 to-blue-50 border border-primary-200 rounded-lg p-8">
        <h2 class="text-xl font-bold text-primary-900 mb-4">📞 Need More Help?</h2>
        <div class="text-primary-800 space-y-2">
          <p>
            <strong>For technical support:</strong>
            Contact your system administrator or IT team.
          </p>
          <p>
            <strong>For feature requests:</strong>
            Please submit requests to your administrator.
          </p>
          <p>
            <strong>System Version:</strong>
            {{ systemVersion }}
          </p>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { settingsApi } from '@/utils/api'

const companyName = ref('LumiSync')
const systemVersion = ref('1.0.0')

async function loadCompanyInfo() {
  try {
    const settings = await settingsApi.getSettings(null).catch(() => ({}))
    if (settings && typeof settings === 'object') {
      companyName.value = settings.company_name || 'LumiSync'
    }
  } catch (e) {
    console.warn('Could not load company settings:', e.message)
  }
}

onMounted(() => {
  loadCompanyInfo()
})
</script>
