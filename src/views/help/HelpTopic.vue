<template>
  <div class="max-w-3xl mx-auto">
    <!-- Breadcrumb -->
    <div class="flex items-center gap-2 text-sm text-gray-400 mb-6">
      <router-link to="/help" class="hover:text-primary-600">Help Center</router-link>
      <span>›</span>
      <span v-if="article" class="text-gray-500">{{ article.category }}</span>
      <span>›</span>
      <span class="text-gray-700 font-medium">{{ article?.title ?? 'Article' }}</span>
    </div>

    <!-- Not found -->
    <div v-if="!article" class="card text-center py-16">
      <p class="text-4xl mb-4">🔍</p>
      <p class="text-gray-600 font-medium">Article not found</p>
      <router-link to="/help" class="btn-primary mt-4 inline-block">Back to Help Center</router-link>
    </div>

    <!-- Article -->
    <div v-else class="card">
      <!-- Article header -->
      <div class="flex items-start gap-4 mb-6 pb-6 border-b border-gray-100">
        <div class="w-12 h-12 rounded-xl bg-primary-50 flex items-center justify-center text-2xl flex-shrink-0">
          {{ article.icon }}
        </div>
        <div>
          <span class="text-xs font-medium text-primary-600 bg-primary-50 px-2 py-0.5 rounded-full">
            {{ article.category }}
          </span>
          <h1 class="text-xl font-bold text-gray-900 mt-2">{{ article.title }}</h1>
          <p class="text-sm text-gray-500 mt-1">{{ article.summary }}</p>
        </div>
      </div>

      <!-- Rendered markdown content -->
      <div class="prose prose-sm max-w-none" v-html="renderedContent" />

      <!-- Related articles -->
      <div v-if="related.length" class="mt-8 pt-6 border-t border-gray-100">
        <h3 class="text-sm font-semibold text-gray-600 mb-3">Related Articles</h3>
        <div class="space-y-2">
          <router-link
            v-for="r in related"
            :key="r.id"
            :to="`/help/${r.id}`"
            class="flex items-center gap-2 text-sm text-primary-600 hover:text-primary-800 hover:underline"
          >
            <span>{{ r.icon }}</span>
            <span>{{ r.title }}</span>
          </router-link>
        </div>
      </div>

      <!-- Navigation buttons -->
      <div class="flex items-center justify-between mt-8 pt-6 border-t border-gray-100">
        <router-link to="/help" class="btn-secondary btn-sm">← Back to Help Center</router-link>
        <button @click="window.print()" class="btn-secondary btn-sm no-print">🖨️ Print</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { HELP_ARTICLES } from './helpContent.js'

const route = useRoute()

const article = computed(() =>
  HELP_ARTICLES.find((a) => a.id === route.params.topic)
)

// Simple markdown → HTML renderer (no external dependency needed)
const renderedContent = computed(() => {
  if (!article.value) return ''
  return renderMarkdown(article.value.content)
})

const related = computed(() => {
  if (!article.value) return []
  return HELP_ARTICLES.filter(
    (a) => a.id !== article.value.id && a.category === article.value.category
  ).slice(0, 4)
})

function renderMarkdown(md) {
  return md
    .trim()
    // Headings
    .replace(/^### (.+)$/gm, '<h3 class="text-base font-semibold text-gray-800 mt-6 mb-2">$1</h3>')
    .replace(/^## (.+)$/gm, '<h2 class="text-lg font-bold text-gray-900 mt-8 mb-3 pb-1 border-b border-gray-100">$1</h2>')
    // Bold
    .replace(/\*\*(.+?)\*\*/g, '<strong class="font-semibold text-gray-900">$1</strong>')
    // Inline code
    .replace(/`(.+?)`/g, '<code class="bg-gray-100 text-primary-700 px-1.5 py-0.5 rounded text-xs font-mono">$1</code>')
    // Code blocks
    .replace(/```[\w]*\n([\s\S]*?)```/g, '<pre class="bg-gray-900 text-green-400 rounded-lg p-4 text-xs overflow-x-auto my-4 font-mono"><code>$1</code></pre>')
    // Tables
    .replace(/\|(.+)\|\n\|[-| :]+\|\n((?:\|.+\|\n?)+)/g, (_match, header, rows) => {
      const ths = header.split('|').filter(Boolean).map(h => `<th class="px-4 py-2 text-left text-xs font-semibold text-gray-500 uppercase bg-gray-50">${h.trim()}</th>`).join('')
      const trs = rows.trim().split('\n').map(row => {
        const tds = row.split('|').filter(Boolean).map(d => `<td class="px-4 py-2 text-sm text-gray-700 border-t border-gray-100">${d.trim()}</td>`).join('')
        return `<tr>${tds}</tr>`
      }).join('')
      return `<div class="overflow-x-auto my-4 rounded-lg border border-gray-200"><table class="w-full"><thead><tr>${ths}</tr></thead><tbody>${trs}</tbody></table></div>`
    })
    // Horizontal rule
    .replace(/^---$/gm, '<hr class="my-6 border-gray-200" />')
    // Blockquote / callout
    .replace(/^> (.+)$/gm, '<div class="bg-yellow-50 border-l-4 border-yellow-400 pl-4 pr-3 py-2 my-3 text-sm text-yellow-900 rounded-r-lg">$1</div>')
    // Checkbox list items
    .replace(/^- \[ \] (.+)$/gm, '<li class="flex items-center gap-2 text-sm text-gray-700 py-0.5"><span class="w-4 h-4 border-2 border-gray-300 rounded inline-flex flex-shrink-0"></span>$1</li>')
    .replace(/^- \[x\] (.+)$/gm, '<li class="flex items-center gap-2 text-sm text-gray-500 py-0.5 line-through"><span class="w-4 h-4 bg-green-500 border-2 border-green-500 rounded inline-flex flex-shrink-0 text-white text-xs items-center justify-center">✓</span>$1</li>')
    // Unordered list items
    .replace(/^- (.+)$/gm, '<li class="text-sm text-gray-700 py-0.5 pl-2 before:content-[\'•\'] before:mr-2 before:text-primary-400">$1</li>')
    // Wrap consecutive <li> in <ul>
    .replace(/(<li.*<\/li>\n?)+/g, m => `<ul class="space-y-1 my-3 ml-2">${m}</ul>`)
    // Paragraphs (blank line separated)
    .replace(/\n\n(?!<)/g, '</p><p class="text-sm text-gray-700 leading-relaxed mt-3">')
    .replace(/^(?!<)/, '<p class="text-sm text-gray-700 leading-relaxed">')
    .replace(/(?!>)$/, '</p>')
}
</script>

<style>
/* Extra prose styles injected by the markdown renderer */
.prose h2:first-child { margin-top: 0; }
</style>
