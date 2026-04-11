import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '@/utils/api'

export const useAuthStore = defineStore('auth', () => {
  const user = ref(null)
  const token = ref(localStorage.getItem('auth_token') || null)
  const loading = ref(false)
  const error = ref(null)

  const isAuthenticated = computed(() => !!token.value && !!user.value)
  const role = computed(() => user.value?.role ?? null)

  function can(allowedRoles) {
    return allowedRoles.includes(role.value)
  }

  async function login(username, password) {
    loading.value = true
    error.value = null
    try {
      const res = await authApi.login(username, password)
      token.value = res.token
      user.value = res.user
      localStorage.setItem('auth_token', res.token)
      return true
    } catch (e) {
      error.value = e.message || 'Login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    if (token.value) {
      try {
        await authApi.logout(token.value)
      } catch (_) {}
    }
    token.value = null
    user.value = null
    localStorage.removeItem('auth_token')
  }

  async function restoreSession() {
    const saved = localStorage.getItem('auth_token')
    if (!saved) return false
    try {
      const u = await authApi.getCurrentUser(saved)
      token.value = saved
      user.value = u
      return true
    } catch (_) {
      localStorage.removeItem('auth_token')
      token.value = null
      user.value = null
      return false
    }
  }

  return { user, token, loading, error, isAuthenticated, role, can, login, logout, restoreSession }
})
