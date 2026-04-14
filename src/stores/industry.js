import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getIndustryConfig } from '@/utils/industryConfig'

export const useIndustryStore = defineStore('industry', () => {
  // State
  const currentIndustry = ref(null)
  const industryConfig = ref(null)
  const userIndustries = ref([]) // User can have access to multiple industries

  // Computed
  const industryName = computed(() => industryConfig.value?.name || 'Unknown Industry')
  const industryIcon = computed(() => industryConfig.value?.icon || '🏢')
  const industryColor = computed(() => industryConfig.value?.color || '#3b82f6')

  // Methods
  function setIndustry(industryId) {
    currentIndustry.value = industryId
    industryConfig.value = getIndustryConfig(industryId)
    localStorage.setItem('selectedIndustry', industryId)
  }

  function getSelectedIndustry() {
    return currentIndustry.value
  }

  function hasFeature(featureName) {
    if (!industryConfig.value) return false
    return industryConfig.value.features[featureName] === true
  }

  function getCustomFields() {
    if (!industryConfig.value) return {}
    return industryConfig.value.customFields || {}
  }

  function getDefaultCategories() {
    if (!industryConfig.value) return []
    return industryConfig.value.defaultCategories || []
  }

  function loadSavedIndustry() {
    const saved = localStorage.getItem('selectedIndustry')
    if (saved) {
      setIndustry(saved)
    }
  }

  return {
    currentIndustry,
    industryConfig,
    userIndustries,
    industryName,
    industryIcon,
    industryColor,
    setIndustry,
    getSelectedIndustry,
    hasFeature,
    getCustomFields,
    getDefaultCategories,
    loadSavedIndustry,
  }
})
