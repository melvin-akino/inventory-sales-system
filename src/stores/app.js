import { defineStore } from 'pinia'
import { ref } from 'vue'
import { settingsApi } from '@/utils/api'
import { useAuthStore } from './auth'

export const useAppStore = defineStore('app', () => {
  const settings = ref({})
  const sidebarOpen = ref(true)
  const notifications = ref([])

  async function loadSettings() {
    const auth = useAuthStore()
    if (!auth.token) return
    try {
      settings.value = await settingsApi.getSettings(auth.token)
    } catch (_) {}
  }

  function notify(message, type = 'success', duration = 3500) {
    const id = Date.now()
    notifications.value.push({ id, message, type })
    setTimeout(() => {
      notifications.value = notifications.value.filter((n) => n.id !== id)
    }, duration)
  }

  function toggleSidebar() {
    sidebarOpen.value = !sidebarOpen.value
  }

  function getSetting(key, fallback = '') {
    return settings.value[key] ?? fallback
  }

  return { settings, sidebarOpen, notifications, loadSettings, notify, toggleSidebar, getSetting }
})
