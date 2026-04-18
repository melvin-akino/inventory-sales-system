<template>
  <div v-if="industry" class="bg-white rounded-lg shadow overflow-hidden">
    <!-- Header with Industry Color -->
    <div class="h-32 rounded-t-lg" :style="{ backgroundColor: industry.color }"></div>

    <div class="px-6 pb-6">
      <!-- Industry Info -->
      <div class="flex items-start gap-4 -mt-16 relative z-10 mb-6">
        <div
          class="w-20 h-20 flex items-center justify-center text-5xl rounded-lg bg-white shadow-lg border-4"
          :style="{ borderColor: industry.color }}$"
        >
          {{ industryIcon }}
        </div>

        <div class="flex-1 pt-2">
          <h2 class="text-2xl font-bold text-gray-900">{{ industry.name }}</h2>
          <p class="text-sm text-gray-600 mt-1">{{ industry.description }}</p>
          <div class="flex items-center gap-2 mt-3">
            <span
              class="inline-block w-3 h-3 rounded-full"
              :style="{ backgroundColor: industry.color }}$"
            ></span>
            <span class="text-xs font-medium text-gray-500 uppercase">{{ industry.code }}</span>
          </div>
        </div>
      </div>

      <!-- Features -->
      <div class="mb-6">
        <h3 class="text-sm font-semibold text-gray-700 mb-3 uppercase">Enabled Features</h3>
        <div class="grid grid-cols-2 gap-2">
          <label v-for="(enabled, feature) in features" :key="feature" class="flex items-center gap-2">
            <input type="checkbox" :checked="enabled" disabled class="w-4 h-4 rounded" />
            <span class="text-sm text-gray-700 capitalize">{{ feature.replace(/_/g, ' ') }}</span>
          </label>
        </div>
      </div>

      <!-- Custom Attributes -->
      <div v-if="attributes.length > 0">
        <h3 class="text-sm font-semibold text-gray-700 mb-3 uppercase">Custom Product Attributes</h3>
        <div class="space-y-2">
          <div
            v-for="attr in attributes"
            :key="attr.id"
            class="flex items-start justify-between p-3 bg-gray-50 rounded-lg"
          >
            <div class="flex-1">
              <div class="text-sm font-medium text-gray-900">{{ attr.attribute_label }}</div>
              <div class="text-xs text-gray-500 mt-1">
                <span class="inline-block px-2 py-1 bg-gray-200 rounded">{{ attr.attribute_type }}</span>
                <span v-if="attr.is_required" class="ml-2 inline-block px-2 py-1 bg-red-100 text-red-700 rounded text-xs"
                  >Required</span
                >
              </div>
            </div>
            <span class="text-xs text-gray-400">{{ attr.attribute_name }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { getIndustryConfig } from '@/utils/industryConfig'

const props = defineProps({
  industryCode: {
    type: String,
    required: true,
  },
  attributes: {
    type: Array,
    default: () => [],
  },
})

const industry = computed(() => getIndustryConfig(props.industryCode))
const industryIcon = computed(() => industry.value?.icon || '🏢')
const features = computed(() => industry.value?.features || {})
</script>
