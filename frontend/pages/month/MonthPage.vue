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

    > .columns {
      height: 100%;
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
        <MonthSummary :month-date="monthDate" class="column" />
        <AddEntry class="column" />
      </div>
    </main>
  </div>
</template>
