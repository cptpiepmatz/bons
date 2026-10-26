<script setup lang="ts">
import type { Dayjs } from "dayjs";

import AppHeader from "../../components/AppHeader.vue";
import { useMonthSpending } from "../../composables/useMonthSpending.ts";
import AddEntry from "./AddEntry.vue";
import MonthSummary from "./MonthSummary.vue";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const { spendingEntries, isLoading, error, refresh } = useMonthSpending(() =>
  monthDate
);
</script>

<style lang="scss" scoped>
.layout {
  height: 100%;
  width: 100%;

  > hr {
    background-color: var(--bulma-border);
  }

  @media (min-width: 769px) {
    main {
      min-height: 0;

      > .columns {
        height: 100%;
        min-height: 0;

        > .column {
          min-height: 0;

          > :deep(.card) {
            max-height: 100%;
            overflow-y: auto;
            overscroll-behavior-y: contain;
          }
        }
      }
    }
  }
}
</style>

<template>
  <div class="layout is-flex is-flex-direction-column">
    <AppHeader class="is-flex-shrink-0" />
    <hr class="m-0 is-flex-shrink-0">
    <main class="is-flex is-flex-grow-1 is-justify-content-center">
      <div class="columns content-width m-0">
        <div class="column is-two-thirds pr-0">
          <MonthSummary
            :month-date="monthDate"
            :spending-entries="spendingEntries"
            :is-loading="isLoading"
            :error="error"
            @retry="refresh"
            @deleted="refresh"
          />
        </div>
        <div class="column is-one-third">
          <AddEntry :month-date="monthDate" @created="refresh" />
        </div>
      </div>
    </main>
  </div>
</template>
