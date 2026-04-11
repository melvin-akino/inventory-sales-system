<template>
  <Modal v-model="modelValue" :title="title" width="400px">
    <p class="text-sm text-gray-600">{{ message }}</p>
    <template #footer>
      <button class="btn-secondary" @click="$emit('update:modelValue', false)">Cancel</button>
      <button :class="confirmClass" @click="confirm">{{ confirmLabel }}</button>
    </template>
  </Modal>
</template>

<script setup>
import Modal from './Modal.vue'

const props = defineProps({
  modelValue: Boolean,
  title: { type: String, default: 'Confirm Action' },
  message: { type: String, default: 'Are you sure?' },
  confirmLabel: { type: String, default: 'Confirm' },
  danger: { type: Boolean, default: false },
})
const emit = defineEmits(['update:modelValue', 'confirm'])
const confirmClass = props.danger ? 'btn-danger' : 'btn-primary'

function confirm() {
  emit('confirm')
  emit('update:modelValue', false)
}
</script>
