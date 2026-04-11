<template>
  <router-view />

  <!-- Global notifications toast -->
  <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <transition-group name="toast">
      <div
        v-for="n in appStore.notifications"
        :key="n.id"
        class="pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg text-sm font-medium min-w-[240px] max-w-sm"
        :class="{
          'bg-green-600 text-white': n.type === 'success',
          'bg-red-600 text-white': n.type === 'error',
          'bg-yellow-500 text-white': n.type === 'warning',
          'bg-blue-600 text-white': n.type === 'info',
        }"
      >
        <span class="text-lg">
          {{ n.type === 'success' ? '✓' : n.type === 'error' ? '✕' : n.type === 'warning' ? '⚠' : 'ℹ' }}
        </span>
        {{ n.message }}
      </div>
    </transition-group>
  </div>
</template>

<script setup>
import { useAppStore } from '@/stores/app'
const appStore = useAppStore()
</script>

<style>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(40px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px);
}
</style>
