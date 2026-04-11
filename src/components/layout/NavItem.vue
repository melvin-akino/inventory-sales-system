<template>
  <router-link
    :to="to"
    class="flex items-center gap-3 mx-2 px-3 py-2 rounded-lg text-sm transition-colors duration-150"
    :class="isActive
      ? 'bg-primary-600 text-white'
      : 'text-gray-400 hover:bg-gray-800 hover:text-white'"
  >
    <span class="text-base flex-shrink-0">{{ icon }}</span>
    <span v-if="appStore.sidebarOpen" class="truncate font-medium">{{ label }}</span>
  </router-link>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAppStore } from '@/stores/app'

const props = defineProps({ to: String, icon: String, label: String })
const route = useRoute()
const appStore = useAppStore()
const isActive = computed(() => route.path === props.to || route.path.startsWith(props.to + '/'))
</script>
