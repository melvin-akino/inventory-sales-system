<template>
  <div>
    <!-- Header -->
    <div class="page-header">
      <h2 class="page-title">Help Center</h2>
      <span class="text-sm text-gray-500">LumiSync v1.0</span>
    </div>

    <!-- Search -->
    <div class="card mb-6">
      <div class="max-w-xl mx-auto text-center py-2">
        <p class="text-gray-500 text-sm mb-3">Search for a topic or browse categories below</p>
        <input
          v-model="query"
          class="input text-base"
          placeholder="🔍  e.g. how to add a product, reset password, void a sale…"
          @input="search"
          autofocus
        />
      </div>
    </div>

    <!-- Search Results -->
    <div v-if="query && results.length" class="card mb-6">
      <h3 class="text-sm font-semibold text-gray-600 mb-3">
        {{ results.length }} result{{ results.length !== 1 ? 's' : '' }} for "{{ query }}"
      </h3>
      <div class="space-y-2">
        <router-link
          v-for="r in results"
          :key="r.id"
          :to="`/help/${r.id}`"
          class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors block"
        >
          <span class="text-xl flex-shrink-0 mt-0.5">{{ r.icon }}</span>
          <div>
            <p class="text-sm font-semibold text-gray-900">{{ r.title }}</p>
            <p class="text-xs text-gray-500 mt-0.5">{{ r.summary }}</p>
            <span class="text-xs text-primary-600 mt-1 inline-block">{{ r.category }}</span>
          </div>
        </router-link>
      </div>
    </div>
    <div v-else-if="query && !results.length" class="card mb-6 text-center py-6 text-gray-400">
      No results found for "{{ query }}". Try different keywords.
    </div>

    <!-- Category Cards -->
    <div v-if="!query" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mb-6">
      <div
        v-for="cat in categories"
        :key="cat.id"
        class="card hover:shadow-md transition-shadow cursor-pointer border-2 hover:border-primary-200"
      >
        <div class="flex items-center gap-3 mb-4">
          <div class="w-10 h-10 rounded-xl flex items-center justify-center text-2xl" :class="cat.bg">
            {{ cat.icon }}
          </div>
          <div>
            <h3 class="font-semibold text-gray-900 text-sm">{{ cat.name }}</h3>
            <p class="text-xs text-gray-400">{{ cat.articles.length }} articles</p>
          </div>
        </div>
        <ul class="space-y-1">
          <li v-for="article in cat.articles.slice(0, 4)" :key="article.id">
            <router-link
              :to="`/help/${article.id}`"
              class="text-sm text-primary-600 hover:text-primary-800 hover:underline block py-0.5"
            >
              {{ article.title }}
            </router-link>
          </li>
          <li v-if="cat.articles.length > 4" class="text-xs text-gray-400 pt-1">
            +{{ cat.articles.length - 4 }} more…
          </li>
        </ul>
      </div>
    </div>

    <!-- Quick Links -->
    <div v-if="!query" class="card">
      <h3 class="text-sm font-semibold text-gray-700 mb-4">Quick Reference</h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <router-link
          v-for="q in quickLinks"
          :key="q.id"
          :to="`/help/${q.id}`"
          class="flex flex-col items-center gap-2 p-4 rounded-xl bg-gray-50 hover:bg-primary-50 hover:border-primary-200 border-2 border-transparent transition-all text-center"
        >
          <span class="text-3xl">{{ q.icon }}</span>
          <span class="text-xs font-medium text-gray-700">{{ q.title }}</span>
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { HELP_ARTICLES, HELP_CATEGORIES } from './helpContent.js'

const query = ref('')
const results = ref([])

const categories = HELP_CATEGORIES
const quickLinks = [
  { id: 'getting-started', title: 'Getting Started', icon: '🚀' },
  { id: 'pos-guide', title: 'Making a Sale', icon: '🛒' },
  { id: 'invoice-print', title: 'Print Invoice/OR', icon: '🖨️' },
  { id: 'stock-adjustment', title: 'Adjust Stock', icon: '⚖️' },
  { id: 'user-roles', title: 'User Roles', icon: '👤' },
  { id: 'reports-overview', title: 'Reports Guide', icon: '📊' },
  { id: 'vat-report', title: 'VAT / BIR Report', icon: '🏛️' },
  { id: 'troubleshooting', title: 'Troubleshooting', icon: '🔧' },
]

function search() {
  if (!query.value.trim()) { results.value = []; return }
  const q = query.value.toLowerCase()
  results.value = HELP_ARTICLES.filter(
    (a) =>
      a.title.toLowerCase().includes(q) ||
      a.summary.toLowerCase().includes(q) ||
      a.tags.some((t) => t.includes(q))
  ).slice(0, 12)
}
</script>
