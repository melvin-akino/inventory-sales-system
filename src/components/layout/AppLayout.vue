<template>
  <div class="flex h-screen overflow-hidden bg-gray-50">
    <Sidebar />
    <div class="flex-1 flex flex-col overflow-hidden">
      <Header />
      <main class="flex-1 overflow-y-auto p-6">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup>
import Sidebar from './Sidebar.vue'
import Header from './Header.vue'
import { useAppStore } from '@/stores/app'
import { onMounted } from 'vue'

const appStore = useAppStore()
onMounted(() => appStore.loadSettings())
</script>
