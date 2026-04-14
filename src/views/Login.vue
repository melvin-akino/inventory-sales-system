<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-900 via-primary-900 to-gray-900 flex items-center justify-center p-4">
    <div class="w-full max-w-sm">
      <!-- Logo & Company Info -->
      <div class="text-center mb-8">
        <!-- Company Logo or Avatar -->
        <div v-if="companyLogo" class="mb-4 flex justify-center">
          <img :src="companyLogo" :alt="companyName" class="h-20 w-20 object-contain" />
        </div>
        <div v-else class="w-20 h-20 rounded-2xl bg-primary-500 mx-auto flex items-center justify-center text-white text-4xl font-bold mb-4 shadow-xl">
          {{ companyInitial }}
        </div>
        
        <!-- Company Name -->
        <h1 class="text-3xl font-bold text-white">{{ companyName }}</h1>
        
        <!-- Company Subtitle/Description -->
        <p class="text-gray-300 text-sm mt-2">{{ companySubtitle }}</p>
        
        <!-- Company Address/Location -->
        <p class="text-gray-400 text-xs mt-1">{{ companyAddress }}</p>
      </div>

      <!-- Card -->
      <div class="bg-white rounded-2xl shadow-2xl p-8">
        <h2 class="text-lg font-semibold text-gray-900 mb-6">Sign in to your account</h2>

        <form @submit.prevent="handleLogin" class="space-y-4">
          <div>
            <label class="label">Username</label>
            <input
              v-model="form.username"
              type="text"
              class="input"
              placeholder="Enter username"
              autocomplete="username"
              required
            />
          </div>

          <div>
            <label class="label">Password</label>
            <div class="relative">
              <input
                v-model="form.password"
                :type="showPassword ? 'text' : 'password'"
                class="input pr-10"
                placeholder="Enter password"
                autocomplete="current-password"
                required
              />
              <button
                type="button"
                @click="showPassword = !showPassword"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
              >{{ showPassword ? '🙈' : '👁️' }}</button>
            </div>
          </div>

          <div v-if="authStore.error" class="bg-red-50 border border-red-200 text-red-700 text-sm rounded-lg px-4 py-3">
            {{ authStore.error }}
          </div>

          <button
            type="submit"
            class="btn-primary w-full py-2.5"
            :disabled="authStore.loading"
          >
            <span v-if="authStore.loading">Signing in…</span>
            <span v-else>Sign In</span>
          </button>
        </form>

        <p class="text-center text-xs text-gray-500 mt-6">
          Default login: admin / Admin@123
        </p>
        <p class="text-center text-xs text-gray-600 mt-2">
          <router-link to="/help" class="text-primary-500 hover:text-primary-600 underline">Need help?</router-link>
        </p>
      </div>

      <p class="text-center text-xs text-gray-500 mt-6">
        {{ companyDisplayText }}
      </p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { settingsApi } from '@/utils/api'

const router = useRouter()
const authStore = useAuthStore()
const showPassword = ref(false)
const form = ref({ username: '', password: '' })

// Company info
const companyName = ref('LumiSync')
const companyAddress = ref('Philippines')
const companySubtitle = ref('Inventory & Sales System')
const companyLogo = ref('')

const companyInitial = computed(() => {
  const parts = companyName.value.trim().split(' ')
  return parts.map(p => p.charAt(0).toUpperCase()).slice(0, 2).join('')
})
const companyDisplayText = computed(() => `${companyName.value} · ${companyAddress.value}`)

async function loadCompanyInfo() {
  try {
    // Load settings without token (should be public or we get defaults)
    const settings = await settingsApi.getSettings(null).catch(() => ({}))
    
    if (settings && typeof settings === 'object') {
      companyName.value = settings.company_name || 'LumiSync'
      companyAddress.value = settings.company_address || 'Philippines'
      companySubtitle.value = settings.company_subtitle || 'Inventory & Sales System'
      
      // Generate avatar with company initial if no logo
      if (!settings.company_logo_url) {
        companyLogo.value = ''
      } else {
        companyLogo.value = settings.company_logo_url
      }
    }
  } catch (e) {
    // If it fails, keep defaults
    console.warn('Could not load company settings:', e.message)
  }
}

async function handleLogin() {
  const ok = await authStore.login(form.value.username, form.value.password)
  if (ok) router.push('/dashboard')
}

onMounted(async () => {
  await loadCompanyInfo()
})
</script>
