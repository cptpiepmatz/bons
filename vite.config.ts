import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";

export default defineConfig({
  root: "frontend",
  plugins: [vue()],
  css: {
    preprocessorOptions: {
      scss: {
        quietDeps: true,
        silenceDeprecations: ["if-function"],
      },
    },
  },
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      allow: [".."],
    },
  },
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    chunkSizeWarningLimit: 600,
  },
});
