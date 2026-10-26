<script setup lang="ts">
import {
  PhCaretDown,
  PhCaretLeft,
  PhCaretRight,
  PhPencilSimple,
} from "@phosphor-icons/vue";
import dayjs, { type Dayjs } from "dayjs";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

import { SpendingResponse } from "../../../bindings/SpendingResponse";
import CategoryIcon from "../../components/CategoryIcon.vue";
import { useAuth } from "../../composables/useAuth.ts";
import { useCurrency } from "../../composables/useCurrency";
import { useDropdown } from "../../composables/useDropdown.ts";
import { categoryDisplayNames } from "../../generated/categories";
import { Money } from "../../models/money";
import { getMonth } from "../../models/month.ts";
import {
  sortOptionDisplayNames,
  type SortOptions,
  sortOptions,
} from "../../models/sortOptions";
import { summarizeSpending } from "../../models/spendingSummary.ts";

const { monthDate, spendingEntries, isLoading, error } = defineProps<{
  monthDate: Dayjs;
  spendingEntries: readonly SpendingResponse[];
  isLoading: boolean;
  error?: string;
}>();
const emit = defineEmits<{ retry: [] }>();
const { currencyFormatter } = useCurrency();
const { user } = useAuth();
const router = useRouter();

// state
const selectedSortOption = ref<SortOptions>("newest");

// bindings
const monthDateInput = ref<HTMLInputElement>();
const sortDropdown = useDropdown();

// derived state
const sortedSpendingEntries = computed(() =>
  [...spendingEntries].sort((a, b) => {
    switch (selectedSortOption.value) {
      case "newest":
        return b.spent_at - a.spent_at;
      case "oldest":
        return a.spent_at - b.spent_at;
      case "highest":
        return b.spent_cents - a.spent_cents;
      case "lowest":
        return a.spent_cents - b.spent_cents;
    }
  })
);

const summary = computed(() => summarizeSpending(spendingEntries));

const spentAtFormatter = new Intl.DateTimeFormat("de-DE", {
  day: "numeric",
  month: "short",
});

// actions
function navigateMonth(date: Dayjs) {
  return router.push({
    name: "month",
    params: {
      year: date.format("YYYY"),
      month: getMonth(date),
    },
  });
}

function selectMonth() {
  const value = monthDateInput.value?.value;
  if (!value) return router.push("/");

  const date = dayjs(value, "YYYY-MM", true);
  if (!date.isValid()) return;
  return navigateMonth(date);
}
</script>

<template>
  <div class="card" style="min-height: 100%;" @scroll="sortDropdown.close">
    <div class="card-content pb-1">
      <div class="is-flex is-flex-direction-column is-align-items-center">
        <div class="field has-addons">
          <p class="control">
            <button
              class="button is-small"
              @click="navigateMonth(monthDate.subtract(1, 'month'))"
            >
              <span class="icon is-large">
                <PhCaretLeft />
              </span>
            </button>
          </p>
          <input
            class="button is-small is-invisible is-position-absolute"
            type="month"
            ref="monthDateInput"
            :value="monthDate.format('YYYY-MM')"
            @change="selectMonth"
            style="width: 10rem;"
          />
          <p class="control">
            <button
              class="button is-small"
              style="width: 10rem;"
              @click="monthDateInput?.showPicker()"
            >
              <span>{{ monthDate.format("MMMM YYYY") }}</span>
            </button>
          </p>
          <p class="control">
            <button
              class="button is-small"
              @click="navigateMonth(monthDate.add(1, 'month'))"
            >
              <span class="icon is-large">
                <PhCaretRight />
              </span>
            </button>
          </p>
        </div>
        <div>
          <p class="title">
            {{ !isLoading && !error && currencyFormatter ? summary.total.format(currencyFormatter) : "&ensp;" }}
          </p>
        </div>
        <div>
          <p class="is-size-7">Insgesamt ausgegeben</p>
        </div>
        <div>
          <p v-if="isLoading" role="status">Ausgaben werden geladen …</p>
          <div v-else-if="error" role="alert" class="has-text-danger">
            <p>Ausgaben konnten nicht geladen werden.</p>
            <p class="is-size-7">{{ error }}</p>
            <button class="button is-small mt-2"
              @click="emit('retry')">Erneut versuchen</button>
          </div>
          <p v-else-if="spendingEntries.length">
            {{ spendingEntries.length }} Ausgaben
          </p>
          <p v-else>
            keine Ausgaben
          </p>
        </div>
      </div>
    </div>
    <hr class="m-0">
    <div class="card-content py-3">
      <p class="title is-6 mb-2">Nach Kategorien</p>
      <p v-if="isLoading" class="is-size-7">Kategorien werden geladen …</p>
      <p v-else-if="error"
        class="is-size-7">Keine Kategorieübersicht verfügbar.</p>
      <p v-else-if="!summary.categories.length"
        class="is-size-7">Noch keine Ausgaben in diesem Monat.</p>
      <div v-else class="fixed-grid">
        <div class="grid">
          <div
            v-for="[category, spent] in summary.categories"
            :key="category"
            class="category-summary has-radius-normal py-2 px-4"
          >
            <div
              class="is-flex is-flex-direction-row is-align-items-center is-gap-1">
              <p class="icon is-large">
                <CategoryIcon :category="category" size="2.8em" />
              </p>
              <div class="is-flex is-flex-direction-column">
                <p class="subtitle is-7 mb-0">
                  {{ categoryDisplayNames[category].de }}
                </p>
                <p class="title is-6 mb-0">
                  {{ currencyFormatter && spent.format(currencyFormatter) }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <hr class="m-0">
    <div class="card-content pt-3">
      <p class="level mb-1">
        <span class="level-left">
          <span class="title is-6">Alle Ausgaben</span>
        </span>
        <span class="level-right">
          <div
            :ref="sortDropdown.element"
            class="dropdown is-right"
            :class="{ 'is-active': sortDropdown.isActive.value }"
          >
            <span class="dropdown-trigger">
              <button
                type="button"
                class="level is-gap-1"
                :aria-expanded="sortDropdown.isActive.value"
                @click="sortDropdown.toggle"
              >
                <span class="level-item is-size-7">
                  {{ sortOptionDisplayNames[selectedSortOption].de }}
                </span>
                <span class="icon is-small level-item">
                  <PhCaretDown />
                </span>
              </button>
            </span>
            <div class="dropdown-menu" role="menu">
              <div class="dropdown-content">
                <a
                  v-for="option in sortOptions"
                  :key="option"
                  class="dropdown-item"
                  @click="selectedSortOption = option; sortDropdown.close()"
                >
                  {{ sortOptionDisplayNames[option].de }}
                </a>
              </div>
            </div>
          </div>
        </span>
      </p>
      <div class="table-container">
        <table class="table is-fullwidth is-hoverable">
          <tbody>
            <tr v-for="spending in sortedSpendingEntries" :key="spending.id">
              <td class="is-narrow is-vcentered">
                <p class="
                  icon 
                  is-medium 
                  has-radius-rounded
                  is-clipped
                  has-text-primary 
                  has-background-primary-light
                ">
                  <CategoryIcon :category="spending.category" size="1.5em" />
                </p>
              </td>
              <td class="is-vcentered">
                <div>
                  <p class="spending-description has-text-weight-semibold">
                    {{ spending.description }}
                  </p>
                  <p class="is-size-7 has-text-text level">
                    <span class="level-left is-gap-1">
                      <span>{{ spentAtFormatter.format(dayjs.unix(spending.spent_at).toDate()) }}</span>
                      <span>&middot;</span>
                      <span>{{ spending.user_id === user?.id ? user.name : `User ${spending.user_id}` }}</span>
                      <span>&middot;</span>
                      <span>{{ categoryDisplayNames[spending.category].de }}</span>
                    </span>
                  </p>
                </div>
              </td>
              <td class="
                is-narrow 
                is-vcentered
                has-text-right 
                has-text-weight-extrabold 
                is-size-5
              ">
              {{ currencyFormatter && Money.fromCents(spending.spent_cents).format(currencyFormatter) }}
            </td>
              <td class="is-narrow is-vcentered">
                <a class="icon has-text-current">
                  <PhPencilSimple />
                </a>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.category-summary {
  border: var(--bulma-control-border-width) solid var(--bulma-border);
}

.spending-description {
  font-size: 0.9em;
}
</style>
