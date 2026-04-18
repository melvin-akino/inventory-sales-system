<template>
  <div v-if="visibleIndustries.length > 0" class="flex items-center gap-2 bg-white rounded-lg shadow px-4 py-3">
    <span class="text-sm font-semibold text-gray-700">Industry:</span>

    <div class="relative">
      <button
        @click="open = !open"
        class="flex items-center gap-2 px-3 py-2 rounded-lg border border-gray-300 hover:border-gray-400 transition text-sm font-medium"
        :style="{ color: selectedIndustry?.color }"
      >
        <span>{{ selectedIndustry?.icon || '🏢' }}</span>
        <span>{{ selectedIndustry?.name || 'Select Industry' }}</span>
        <span class="text-xs">▼</span>
      </button>

      <!-- Dropdown Menu -->
      <transition name="fade">
        <div
          v-if="open"
          class="absolute top-full left-0 mt-1 bg-white border border-gray-300 rounded-lg shadow-lg z-50 min-w-[300px]"
        >
          <div class="p-2 space-y-1 max-h-[400px] overflow-y-auto">
            <button
              v-for="ind in visibleIndustries"
              :key="ind.id"
              @click="selectIndustry(ind)"
              class="w-full flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-gray-100 transition text-left text-sm"
              :class="{
                'bg-gray-100 font-semibold': selectedIndustry?.id === ind.id,
              }"
            >
              <span class="text-lg">{{ ind.icon }}</span>
              <div class="flex-1">
                <div class="font-medium text-gray-900">{{ ind.name }}</div>
                <div class="text-xs text-gray-500">{{ ind.description }}</div>
              </div>
              <span
                v-if="selectedIndustry?.id === ind.id"
                class="text-sm font-bold"
                :style="{ color: ind.color }"
              >
                ✓
              </span>
            </button>
          </div>
        </div>
      </transition>
    </div>

    <!-- Selected Industry Badge -->
    <div
      v-if="selectedIndustry"
      class="ml-auto flex items-center gap-2 px-3 py-1 rounded-full text-sm font-medium text-white"
      :style="{ backgroundColor: selectedIndustry.color }}$"
    >
      <span>{{ selectedIndustry.name }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useIndustryStore } from '@/stores/industry'
import { getAllIndustryConfigs } from '@/utils/industryConfig'

const auth = useAuthStore()
const industry = useIndustryStore()

const open = ref(false)

const visibleIndustries = computed(() => {
  // Super admin sees all industries
  // Admin with single industry only sees their own (can't switch in UI)
  if (auth.can(['super_admin'])) {
    return getAllIndustryConfigs()
  }
  return []
})

const selectedIndustry = computed(() => {
  return industry.currentIndustry || auth.userIndustry
})

function selectIndustry(ind) {
  industry.setIndustry(ind)
  open.value = false
  // Optionally emit event to parent to reload data
  window.location.reload() // Simple approach; better to use router or emit event
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
