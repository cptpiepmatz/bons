<script setup lang="ts">
import type { Dayjs } from "dayjs";

import { suggestCategory } from "../services/classifier.ts";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const formattedMonth = monthDate.format("MMMM YYYY");

async function testSuggestCategory() {
  console.log(await suggestCategory("Aldi"));
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
            <button class="button" type="button" @click="testSuggestCategory">
              Test category suggestion
            </button>
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
