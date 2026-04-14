import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

export default defineConfig(({ command, mode }) => {
  const isTauri = process.env.TAURI_PLATFORM !== undefined

  return {
    plugins: [vue()],
    resolve: {
      alias: {
        '@': resolve(__dirname, 'src'),
      },
    },
    // Vite options tailored for Tauri development
    clearScreen: false,
    server: {
      port: 1420,
      strictPort: true,
      proxy: {
        '/api': {
          target: 'http://localhost:3000',
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/api/, ''),
        },
      },
    },
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
      // Tauri uses Chromium on Windows and WebKit on macOS/Linux
      target: isTauri ? (process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13') : 'esnext',
      minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
      sourcemap: !!process.env.TAURI_DEBUG,
      outDir: 'dist',
    },
    define: {
      __IS_TAURI__: isTauri,
      __API_URL__: isTauri ? '"http://localhost:3000"' : '"/api"',
    },
  }
})
