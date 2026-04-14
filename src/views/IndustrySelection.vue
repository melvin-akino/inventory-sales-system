<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
    <!-- Header -->
    <div class="bg-white border-b border-gray-200 shadow-sm">
      <div class="max-w-6xl mx-auto px-6 py-6">
        <h1 class="text-3xl font-bold text-gray-900">Choose Your Industry</h1>
        <p class="text-gray-600 mt-2">Select an industry to set up your business POS and inventory system</p>
      </div>
    </div>

    <!-- Industry Selection Grid -->
    <div class="max-w-6xl mx-auto px-6 py-12">
      <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="industry in industries"
          :key="industry.id"
          @click="selectIndustry(industry.id)"
          class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 cursor-pointer overflow-hidden border-2 border-transparent hover:border-primary-500"
        >
          <!-- Header with Color -->
          <div
            :style="{ backgroundColor: industry.color }"
            class="h-20 flex items-end justify-start p-4"
          >
            <span class="text-5xl">{{ industry.icon }}</span>
          </div>

          <!-- Content -->
          <div class="p-6">
            <h2 class="text-xl font-bold text-gray-900 mb-2">{{ industry.name }}</h2>
            <p class="text-gray-600 text-sm mb-4">{{ industry.description }}</p>

            <!-- Features -->
            <div class="mb-4">
              <h3 class="text-sm font-semibold text-gray-700 mb-2">Features:</h3>
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="(enabled, feature) in industry.features"
                  :key="feature"
                  v-show="enabled"
                  class="inline-block px-2 py-1 bg-blue-50 text-blue-700 text-xs rounded"
                >
                  ✓ {{ formatFeatureName(feature) }}
                </span>
              </div>
            </div>

            <!-- Action Button -->
            <button
              class="w-full py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors font-medium"
            >
              Select {{ industry.name }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Coming Soon -->
    <div class="max-w-6xl mx-auto px-6 py-8 mt-8">
      <div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 class="font-semibold text-blue-900 mb-2">Coming Soon</h3>
        <p class="text-blue-800 text-sm">
          More industries will be added: Supermarket, Restaurant, General Retail, and more.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getAvailableIndustries } from '@/utils/industryConfig'

const router = useRouter()
const authStore = useAuthStore()
const industries = ref([])

onMounted(() => {
  industries.value = getAvailableIndustries()
})

function formatFeatureName(feature) {
  return feature
    .split('_')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

async function selectIndustry(industryId) {
  // Save selected industry to user profile
  authStore.selectedIndustry = industryId
  
  // Redirect to dashboard with industry-specific UI
  router.push({
    name: 'Dashboard',
    query: { industry: industryId }
  })
}
</script>
