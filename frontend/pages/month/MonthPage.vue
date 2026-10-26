<script setup lang="ts">
import { PhPlus } from "@phosphor-icons/vue";
import type { Dayjs } from "dayjs";
import { ref } from "vue";
import { useI18n } from "vue-i18n";

import AppHeader from "../../components/AppHeader.vue";
import { useMediaQuery } from "../../composables/useMediaQuery.ts";
import { useMonthSpending } from "../../composables/useMonthSpending.ts";
import AddEntry from "./AddEntry.vue";
import MonthSummary from "./MonthSummary.vue";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const { t } = useI18n();
const isMobile = useMediaQuery("(max-width: 768px)");
const addEntry = ref<InstanceType<typeof AddEntry>>();
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

.mobile-heading,
.mobile-add-bar {
  display: none;
}

@media (max-width: 768px) {
  .layout {
    height: auto;
    min-height: 100%;

    main {
      display: block !important;
      padding: 1.5rem 1.25rem calc(6rem + env(safe-area-inset-bottom));
    }

    .columns > .column {
      padding: 0 !important;
      min-width: 0;
    }
  }

  .mobile-heading {
    display: block;
    margin-bottom: 1.75rem;

    h1 {
      font-size: 1.65rem;
      font-weight: 750;
      letter-spacing: -.04em;
      color: var(--bulma-text-strong);
    }

    p {
      margin-top: .35rem;
      font-size: .85rem;
      color: var(--bulma-text-weak);
    }
  }

  .mobile-add-bar {
    display: block;
    position: fixed;
    z-index: 10;
    bottom: 0;
    inset-inline: 0;
    padding: .75rem 1.25rem calc(.75rem + env(safe-area-inset-bottom));
    background: var(--bulma-scheme-main);
    border-top: 1px solid var(--bulma-border-weak);

    .button {
      min-height: 3rem;
      font-weight: 600;
    }
  }
}
</style>

<template>
  <div class="layout is-flex is-flex-direction-column">
    <AppHeader class="is-flex-shrink-0" />
    <hr class="m-0 is-flex-shrink-0">
    <main class="is-flex is-flex-grow-1 is-justify-content-center">
      <div class="mobile-heading">
        <h1>{{ t("spending.title") }}</h1>
        <p>{{ t("common.tagline") }}</p>
      </div>
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
          <AddEntry ref="addEntry" :modal="isMobile" @created="refresh" />
        </div>
      </div>
    </main>
    <div class="mobile-add-bar">
      <button type="button" class="button is-primary is-fullwidth"
        @click="addEntry?.open()">
        <span class="icon"><PhPlus weight="bold" /></span>
        <span>{{ t("spending.add") }}</span>
      </button>
    </div>
  </div>
</template>
