import { defineConfig, tsExtractor } from "@primp/primp";
import {
  declarationText,
  nodePrefix,
  packageSource,
  parentPath,
  sideEffect,
  sourcePath,
} from "@primp/primp/rules/imports";
import * as separators from "@primp/primp/rules/separators";
import { vueExtractor } from "@primp/vue";

export default defineConfig({
  extractors: [tsExtractor, vueExtractor],
  sortImports: [
    sideEffect,
    nodePrefix,
    packageSource,
    parentPath,
    sourcePath,
    declarationText,
  ],
  separateBy: [
    separators.sideEffect,
    separators.nodePrefix,
    separators.packageSource,
  ],
});
