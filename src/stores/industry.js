import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { industriesApi } from '@/utils/api'
import { getIndustryConfig, getAllIndustryConfigs } from '@/utils/industryConfig'
import { useAuthStore } from './auth'

export const useIndustryStore = defineStore('industry', () => {
  // State
  const currentIndustry = ref(null)
  const industryData = ref(null) // Data from backend (id, code, name, description, color, attributes)
  const availableIndustries = ref([]) // All available industries
  const customAttributes = ref([]) // Industry-specific attributes
  const loading = ref(false)
  const error = ref(null)

  // Computed
  const industryId = computed(() => currentIndustry.value?.id || industryData.value?.id)
  const industryCode = computed(() => currentIndustry.value?.code || industryData.value?.code)
  const industryName = computed(() => currentIndustry.value?.name || industryData.value?.name || 'Unknown')
  const industryColor = computed(() => currentIndustry.value?.color || industryData.value?.color || '#3b82f6')
  const industryDescription = computed(() => currentIndustry.value?.description || industryData.value?.description || '')

  const localConfig = computed(() => getIndustryConfig(industryCode.value))

  // Methods
  async function loadUserIndustry() {
    const auth = useAuthStore()
    if (auth.userIndustry) {
      currentIndustry.value = auth.userIndustry
      // Optionally fetch full details with attributes
      await loadIndustryDetails(auth.userIndustry.id)
    }
  }

  async function loadIndustries(token) {
    loading.value = true
    error.value = null
    try {
      const industries = await industriesApi.getIndustries(token)
      availableIndustries.value = industries
      return industries
    } catch (e) {
      error.value = e.message || 'Failed to load industries'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function loadIndustryDetails(industryId, token = null) {
    loading.value = true
    error.value = null
    try {
      const auth = useAuthStore()
      const t = token || auth.token
      if (!t) throw new Error('No token available')

      const industry = await industriesApi.getIndustry(t, industryId)
      industryData.value = industry
      customAttributes.value = industry.attributes || []
      return industry
    } catch (e) {
      error.value = e.message || 'Failed to load industry details'
      throw e
    } finally {
      loading.value = false
    }
  }

  function setIndustry(industryOrId) {
    if (typeof industryOrId === 'number') {
      // Find by ID in available industries
      const found = availableIndustries.value.find((i) => i.id === industryOrId)
      if (found) {
        currentIndustry.value = found
        localStorage.setItem('selectedIndustry', JSON.stringify(found))
      }
    } else {
      // Already an industry object
      currentIndustry.value = industryOrId
      localStorage.setItem('selectedIndustry', JSON.stringify(industryOrId))
    }
  }

  function loadSavedIndustry() {
    const saved = localStorage.getItem('selectedIndustry')
    if (saved) {
      try {
        const industry = JSON.parse(saved)
        currentIndustry.value = industry
      } catch (_) {
        localStorage.removeItem('selectedIndustry')
      }
    }
  }

  // Feature checks
  function hasFeature(featureName) {
    return localConfig.value?.features[featureName] === true
  }

  // Get custom fields from local config
  function getCustomFields() {
    return localConfig.value?.customFields || {}
  }

  // Get custom fields from backend (industry-specific attributes)
  function getIndustryAttributes() {
    return customAttributes.value || []
  }

  // Get default categories
  function getDefaultCategories() {
    return localConfig.value?.defaultCategories || []
  }

  // Check specific attribute requirement
  function isAttributeRequired(attributeName) {
    const attr = customAttributes.value?.find((a) => a.attribute_name === attributeName)
    return attr?.is_required || false
  }

  async function addAttribute(token, attributeName, attributeLabel, attributeType = 'text', isRequired = false, displayOrder = 0) {
    loading.value = true
    error.value = null
    try {
      const request = {
        industry_id: industryId.value,
        attribute_name: attributeName,
        attribute_label: attributeLabel,
        attribute_type: attributeType,
        is_required: isRequired,
        display_order: displayOrder,
      }
      const newAttr = await industriesApi.addIndustryAttribute(token, request)
      customAttributes.value.push(newAttr)
      return newAttr
    } catch (e) {
      error.value = e.message || 'Failed to add attribute'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    currentIndustry,
    industryData,
    availableIndustries,
    customAttributes,
    loading,
    error,
    industryId,
    industryCode,
    industryName,
    industryColor,
    industryDescription,
    loadUserIndustry,
    loadIndustries,
    loadIndustryDetails,
    setIndustry,
    loadSavedIndustry,
    hasFeature,
    getCustomFields,
    getIndustryAttributes,
    getDefaultCategories,
    isAttributeRequired,
    addAttribute,
  }
})
