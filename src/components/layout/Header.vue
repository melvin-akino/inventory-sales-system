<template>
  <header class="bg-white border-b border-gray-200 px-6 py-3 flex items-center justify-between flex-shrink-0">
    <div class="flex items-center gap-4">
      <button @click="appStore.toggleSidebar()" class="btn-icon">
        <span class="text-xl">☰</span>
      </button>
      <h1 class="text-base font-semibold text-gray-800">{{ routeTitle }}</h1>
    </div>
    <div class="flex items-center gap-3 text-sm text-gray-500">
      <span>{{ currentDate }}</span>
      <span>|</span>
      <span class="font-medium text-gray-700">{{ authStore.user?.full_name }}</span>
    </div>
  </header>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { formatDate } from '@/utils/format'

const route = useRoute()
const authStore = useAuthStore()
const appStore = useAppStore()

const routeTitle = computed(() => route.meta?.title ?? 'LumiSync')
const currentDate = computed(() => formatDate(new Date().toISOString()))
</script>
