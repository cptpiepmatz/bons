<script setup lang="ts">
import { ref } from "vue";

import { suggestCategory } from "../../services/classifier.ts";

const categoryInput = ref("Aldi");
const isClassifying = ref(false);

async function testSuggestCategory() {
  const text = categoryInput.value.trim();
  if (!text || isClassifying.value) return;

  isClassifying.value = true;
  console.time("Category suggestion");
  try {
    const result = await suggestCategory(text);
    if (result === undefined) {
      console.info("No unambiguous category match.");
    } else {
      console.log(result);
    }
  } catch (error) {
    console.error(error);
  } finally {
    console.timeEnd("Category suggestion");
    isClassifying.value = false;
  }
}
</script>

<template>
  <div></div>
</template>
