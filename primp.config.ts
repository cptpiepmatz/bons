import { defineConfig, tsExtractor } from "@primp/primp";
import { vueExtractor } from "@primp/vue";

export default defineConfig({
  extractors: [tsExtractor, vueExtractor],
});
