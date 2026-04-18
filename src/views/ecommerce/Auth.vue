<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4">
    <div class="bg-white rounded-lg shadow-lg p-8 w-full max-w-md">
      <!-- Tabs -->
      <div class="flex gap-4 mb-8 border-b border-gray-200">
        <button
          @click="activeTab = 'login'"
          :class="[
            'pb-4 font-semibold',
            activeTab === 'login'
              ? 'border-b-2 border-blue-600 text-blue-600'
              : 'text-gray-600'
          ]"
        >
          Login
        </button>
        <button
          @click="activeTab = 'register'"
          :class="[
            'pb-4 font-semibold',
            activeTab === 'register'
              ? 'border-b-2 border-blue-600 text-blue-600'
              : 'text-gray-600'
          ]"
        >
          Register
        </button>
      </div>

      <!-- Login Form -->
      <form v-if="activeTab === 'login'" @submit.prevent="handleLogin" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
          <input
            v-model="loginForm.email"
            type="email"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="user@example.com"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Password</label>
          <input
            v-model="loginForm.password"
            type="password"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="••••••••"
          />
        </div>

        <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
          {{ error }}
        </div>

        <button
          type="submit"
          :disabled="isLoading"
          :class="[
            'w-full px-4 py-2 rounded-lg font-semibold text-white',
            isLoading
              ? 'bg-gray-400 cursor-not-allowed'
              : 'bg-blue-600 hover:bg-blue-700'
          ]"
        >
          {{ isLoading ? 'Logging in...' : 'Login' }}
        </button>
      </form>

      <!-- Register Form -->
      <form v-if="activeTab === 'register'" @submit.prevent="handleRegister" class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">First Name</label>
            <input
              v-model="registerForm.firstName"
              type="text"
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="John"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Last Name</label>
            <input
              v-model="registerForm.lastName"
              type="text"
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Doe"
            />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
          <input
            v-model="registerForm.email"
            type="email"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="user@example.com"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Phone (optional)</label>
          <input
            v-model="registerForm.phone"
            type="tel"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="09123456789"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Password (min. 6 characters)</label>
          <input
            v-model="registerForm.password"
            type="password"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="••••••••"
          />
        </div>

        <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
          {{ error }}
        </div>

        <button
          type="submit"
          :disabled="isLoading"
          :class="[
            'w-full px-4 py-2 rounded-lg font-semibold text-white',
            isLoading
              ? 'bg-gray-400 cursor-not-allowed'
              : 'bg-blue-600 hover:bg-blue-700'
          ]"
        >
          {{ isLoading ? 'Creating account...' : 'Create Account' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useEcommerceStore } from '@/stores/ecommerce'

const router = useRouter()
const ecommerceStore = useEcommerceStore()

// State
const activeTab = ref('login')
const isLoading = ref(false)
const error = ref('')

const loginForm = ref({
  email: '',
  password: '',
})

const registerForm = ref({
  firstName: '',
  lastName: '',
  email: '',
  phone: '',
  password: '',
})

// Methods
async function handleLogin() {
  error.value = ''
  isLoading.value = true

  try {
    await ecommerceStore.login(loginForm.value.email, loginForm.value.password)
    router.push('/shop')
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

async function handleRegister() {
  error.value = ''
  isLoading.value = true

  try {
    await ecommerceStore.register(
      registerForm.value.email,
      registerForm.value.password,
      registerForm.value.firstName,
      registerForm.value.lastName,
      registerForm.value.phone
    )
    // Auto-login after registration
    await ecommerceStore.login(registerForm.value.email, registerForm.value.password)
    router.push('/shop')
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}
</script>
