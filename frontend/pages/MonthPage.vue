<script setup lang="ts">
import { ref } from "vue";

import type { Dayjs } from "dayjs";

import { suggestCategory } from "../services/classifier.ts";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const formattedMonth = monthDate.format("MMMM YYYY");
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

<style lang="scss" scoped>
.layout {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;

  @media (min-width: 769px) {
    background-color: var(--bulma-white-ter);

    @media (prefers-color-scheme: dark) {
      background-color: var(--bulma-black-ter);
    }
  }

  > * {
    max-width: 30rem;
  }
}
</style>

<template>
  <div class="layout">
    <div class="columns">
      <div class="column">
        <div class="card">
          <div class="card-content">
            <p>{{ formattedMonth }}</p>
            <form class="mt-4" @submit.prevent="testSuggestCategory">
              <div class="field">
                <label class="label"
                  for="category-input">Store or expense</label>
                <div class="control">
                  <input
                    id="category-input"
                    v-model="categoryInput"
                    class="input"
                    type="text"
                    placeholder="Aldi, Rossmann, Friseur…"
                  />
                </div>
              </div>
              <button
                class="button"
                :class="{ 'is-loading': isClassifying }"
                type="submit"
                :disabled="isClassifying || !categoryInput.trim()"
              >
                Test category suggestion
              </button>
            </form>
          </div>
        </div>
      </div>
      <div class="column">
        <div class="card">
          <div class="card-content">
            right
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
