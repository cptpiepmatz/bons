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

  > :not(main) {
    flex-shrink: 0;
  }

  > hr {
    background-color: var(--bulma-border);
  }

  main {
    flex-grow: 1;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;

    > .columns {
      width: 100%;
      max-width: var(--content-width);
      margin: 0;
    }
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
  <div class="layout">
    <AppHeader />
    <hr class="m-0">
    <main>
      <div class="columns">
        <div class="column is-two-thirds pr-0">
          <MonthSummary :month-date="monthDate" />
        </div>
        <div class="column is-one-third">
          <AddEntry />
        </div>
      </div>
    </main>
  </div>
</template>
