<script setup lang="ts">
import type { Dayjs } from "dayjs";

import AppHeader from "../../components/AppHeader.vue";
import AddEntry from "./AddEntry.vue";
import MonthSummary from "./MonthSummary.vue";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
</script>

<style lang="scss" scoped>
.layout {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;

  main {
    flex-grow: 1;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;

    > .columns {
      height: 100%;
      width: 100%;
      max-width: var(--content-width);
    }
  }
}
</style>

<template>
  <div class="layout">
    <AppHeader />
    <hr class="mt-0">
    <main>
      <div class="columns">
        <div class="column">
          <MonthSummary :month-date="monthDate" />
        </div>
        <div class="column">
          <AddEntry />
        </div>
      </div>
    </main>
  </div>
</template>
