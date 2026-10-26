<script setup lang="ts">
import {
  PhCaretDown,
  PhCaretLeft,
  PhCaretRight,
  PhTrash,
} from "@phosphor-icons/vue";
import dayjs, { type Dayjs } from "dayjs";
import { computed, nextTick, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { SpendingResponse } from "../../../bindings/SpendingResponse";
import CategoryIcon from "../../components/CategoryIcon.vue";
import { useAuth } from "../../composables/useAuth.ts";
import { useCurrency } from "../../composables/useCurrency";
import { useDeleteSpending } from "../../composables/useDeleteSpending.ts";
import { useDropdown } from "../../composables/useDropdown.ts";
import { useLocale } from "../../composables/useLocale.ts";
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
const emit = defineEmits<{ retry: []; deleted: [] }>();
const { t } = useI18n();
const { locale, monthFormatter, shortDateFormatter: spentAtFormatter } =
  useLocale();
const { remove, clearError, isDeleting, error: deleteError } =
  useDeleteSpending();
const { currencyFormatter } = useCurrency();
const { user } = useAuth();
const router = useRouter();

// state
const selectedSortOption = ref<SortOptions>("newest");
const spendingToDelete = ref<SpendingResponse>();

// bindings
const monthDateInput = ref<HTMLInputElement>();
const sortDropdown = useDropdown();
const deleteModal = ref<HTMLDialogElement>();

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

// actions
async function confirmDelete(spending: SpendingResponse) {
  clearError();
  spendingToDelete.value = spending;
  await nextTick();
  deleteModal.value?.showModal();
}

function closeDeleteModal() {
  if (isDeleting.value) return;
  deleteModal.value?.close();
  spendingToDelete.value = undefined;
}

async function deleteSpending() {
  if (!spendingToDelete.value) return;
  if (!await remove(spendingToDelete.value.id)) return;
  closeDeleteModal();
  emit("deleted");
}

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
  <div class="card month-summary" style="min-height: 100%;"
    @scroll="sortDropdown.close">
    <div class="card-content pb-1">
      <div class="is-flex is-flex-direction-column is-align-items-center">
        <div class="field has-addons">
          <p class="control">
            <button
              class="button is-small"
              :aria-label="t('spending.previousMonth')"
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
          <p class="control month-control">
            <button
              class="button is-small month-button"
              @click="monthDateInput?.showPicker()"
            >
              <span>{{ monthFormatter.format(monthDate.toDate()) }}</span>
            </button>
          </p>
          <p class="control">
            <button
              class="button is-small"
              :aria-label="t('spending.nextMonth')"
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
          <p class="is-size-7">{{ t("spending.total") }}</p>
        </div>
        <div>
          <p v-if="isLoading" role="status">{{ t("spending.loading") }}</p>
          <div v-else-if="error" role="alert" class="has-text-danger">
            <p>{{ t("spending.loadError") }}</p>
            <p class="is-size-7">{{ error }}</p>
            <button class="button is-small mt-2"
              @click="emit('retry')">{{ t("common.retry") }}</button>
          </div>
          <p v-else-if="spendingEntries.length">
            {{ t("spending.count", spendingEntries.length) }}
          </p>
          <p v-else>
            {{ t("spending.empty") }}
          </p>
        </div>
      </div>
    </div>
    <hr class="m-0">
    <div class="card-content py-3">
      <p class="title is-6 mb-2">{{ t("spending.byCategory") }}</p>
      <p v-if="isLoading"
        class="is-size-7">{{ t("spending.categoriesLoading") }}</p>
      <p v-else-if="error"
        class="is-size-7">{{ t("spending.categoriesUnavailable") }}</p>
      <p v-else-if="!summary.categories.length"
        class="is-size-7">{{ t("spending.monthEmpty") }}</p>
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
                  {{ categoryDisplayNames[category][locale] }}
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
      <div class="entries-heading level is-mobile mb-1">
        <span class="level-left">
          <span class="title is-6">{{ t("spending.all") }}</span>
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
                class="sort-trigger level is-mobile is-gap-1"
                :aria-expanded="sortDropdown.isActive.value"
                @click="sortDropdown.toggle"
              >
                <span class="level-item is-size-7">
                  {{ sortOptionDisplayNames[selectedSortOption][locale] }}
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
                  role="menuitem"
                  tabindex="0"
                  @keydown.enter="selectedSortOption = option; sortDropdown.close()"
                  @keydown.space.prevent="selectedSortOption = option; sortDropdown.close()"
                  @click="selectedSortOption = option; sortDropdown.close()"
                >
                  {{ sortOptionDisplayNames[option][locale] }}
                </a>
              </div>
            </div>
          </div>
        </span>
      </div>
      <div class="table-container">
        <table class="table is-fullwidth is-hoverable">
          <tbody>
            <tr v-for="spending in sortedSpendingEntries" :key="spending.id"
              class="spending-row">
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
              <td class="spending-details is-vcentered">
                <div>
                  <p class="spending-description has-text-weight-semibold">
                    {{ spending.description }}
                  </p>
                  <p
                    class="spending-metadata is-size-7 has-text-text level is-mobile">
                    <span class="level-left is-gap-1">
                      <span>{{ spentAtFormatter.format(dayjs.unix(spending.spent_at).toDate()) }}</span>
                      <span>&middot;</span>
                      <span>{{ spending.user_id === user?.id ? user.name : t("common.user", { id: spending.user_id }) }}</span>
                      <span class="spending-category">&middot;</span>
                      <span
                        class="spending-category">{{ categoryDisplayNames[spending.category][locale] }}</span>
                    </span>
                  </p>
                </div>
              </td>
              <td class="
                spending-amount
                is-narrow 
                is-vcentered
                has-text-right 
                has-text-weight-extrabold 
                is-size-5
              ">
              {{ currencyFormatter && Money.fromCents(spending.spent_cents).format(currencyFormatter) }}
            </td>
              <td class="is-narrow is-vcentered">
                <button
                  type="button"
                  class="icon delete-spending"
                  :aria-label="t('spending.deleteLabel', { description: spending.description })"
                  @click="confirmDelete(spending)"
                >
                  <PhTrash />
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
  <Teleport to="body">
    <dialog
      ref="deleteModal"
      class="modal delete-modal"
      :class="{ 'is-active': spendingToDelete }"
      aria-labelledby="delete-spending-title"
      aria-describedby="delete-spending-question"
      @cancel.prevent="closeDeleteModal"
    >
      <div class="modal-background" @click="closeDeleteModal"></div>
      <div v-if="spendingToDelete" class="modal-card">
        <header class="modal-card-head">
          <p id="delete-spending-title"
            class="modal-card-title">{{ t("spending.deleteTitle") }}</p>
          <button type="button" class="delete" :aria-label="t('common.close')"
            :disabled="isDeleting" @click="closeDeleteModal"></button>
        </header>
        <section class="modal-card-body">
          <p id="delete-spending-question"
            class="mb-4">{{ t("spending.deleteQuestion") }}</p>
          <div
            class="delete-spending-heading has-background-primary-light has-radius-normal p-4 mb-5">
            <span
              class="icon is-large is-flex-shrink-0 has-text-primary">
              <CategoryIcon :category="spendingToDelete.category"
                size="2.5em" />
            </span>
            <h3
              class="title is-4 has-text-dark has-text-centered mb-0">{{ spendingToDelete.description }}</h3>
          </div>
          <div class="columns is-justify-content-space-around">
            <div class="column is-narrow">
              <p class="has-text-weight-semibold">{{ t("common.amount") }}</p>
              <p>{{ currencyFormatter && Money.fromCents(spendingToDelete.spent_cents).format(currencyFormatter) }}</p>
            </div>
            <div class="column is-narrow">
              <p class="has-text-weight-semibold">{{ t("common.date") }}</p>
              <p>{{ spentAtFormatter.format(dayjs.unix(spendingToDelete.spent_at).toDate()) }}</p>
            </div>
            <div class="column is-narrow">
              <p
                class="has-text-weight-semibold">{{ t("common.enteredBy") }}</p>
              <p>{{ spendingToDelete.user_id === user?.id ? user.name : t("common.user", { id: spendingToDelete.user_id }) }}</p>
            </div>
            <div class="column is-narrow">
              <p class="has-text-weight-semibold">{{ t("common.category") }}</p>
              <p>{{ categoryDisplayNames[spendingToDelete.category][locale] }}</p>
            </div>
          </div>
          <p v-if="deleteError" class="help is-danger mt-3" role="alert">
            {{ t("spending.deleteError") }} {{ deleteError }}
          </p>
        </section>
        <footer class="modal-card-foot is-justify-content-flex-end">
          <div class="buttons">
            <button type="button" class="button" autofocus
              :disabled="isDeleting"
              @click="closeDeleteModal">{{ t("common.cancel") }}</button>
            <button type="button" class="button is-danger"
              :class="{ 'is-loading': isDeleting }" :disabled="isDeleting"
              @click="deleteSpending">{{ t("common.delete") }}</button>
          </div>
        </footer>
      </div>
    </dialog>
  </Teleport>
</template>

<style scoped lang="scss">
.month-button {
  width: 10rem;
}

.category-summary {
  border: var(--bulma-control-border-width) solid var(--bulma-border);
}

.spending-description {
  font-size: 0.9em;
}

.delete-spending {
  color: currentColor;

  &:hover,
  &:focus-visible {
    color: var(--bulma-danger);
  }
}

.delete-modal {
  width: 100%;
  height: 100%;
  max-width: none;
  max-height: none;
  padding: 0;
  border: 0;
  background: transparent;
  color: inherit;
}

.delete-spending-heading {
  display: grid;
  grid-template-columns: 3rem minmax(0, auto) 1rem;
  justify-content: center;
  align-items: center;
  gap: 0.75rem;
}

@media (max-width: 768px) {
  .month-summary {
    min-height: 0 !important;
    border-radius: 0;
    box-shadow: none;

    > .card-content {
      padding-inline: 0;
    }

    > .card-content:first-child {
      padding-top: 0;
      padding-bottom: 1.5rem !important;

      .field.has-addons {
        width: 100%;
        margin-bottom: 1.5rem;
      }

      .month-control {
        flex: 1;
        min-width: 0;
      }

      .month-button {
        width: 100%;
      }

      .button {
        min-height: 2.75rem;
      }
      .title {
        font-size: 2.5rem;
        letter-spacing: -.045em;
        color: var(--bulma-text-strong);
      }
      p {
        color: var(--bulma-text-weak);
        font-size: .8rem;
      }
    }
  }

  .fixed-grid .grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: .65rem;
    margin-top: .85rem;
    margin-bottom: .5rem;
  }

  .category-summary {
    padding: .8rem .65rem !important;
    min-width: 0;

    > div {
      gap: .5rem !important;
    }
    .icon {
      flex-shrink: 0;
      width: 1.8rem;
      height: 1.8rem;
    }
    .icon :deep(svg) {
      width: 1.8rem;
      height: 1.8rem;
    }
    .subtitle {
      font-size: .7rem !important;
      overflow-wrap: anywhere;
    }
    .title {
      font-size: .95rem !important;
      margin-top: .25rem;
      white-space: nowrap;
    }
  }

  .entries-heading {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: .5rem;
    min-height: 2.75rem;

    > .level-left,
     > .level-right {
      margin: 0;
    }
    .title {
      white-space: nowrap;
    }
  }

  .sort-trigger {
    display: flex;
    align-items: center;
    min-height: 2.75rem;
    margin-bottom: 0;

    .level-item {
      margin: 0;
      font-size: .7rem !important;
    }
  }

  .table-container {
    overflow: visible;
  }
  .table,
  .table tbody {
    display: block;
    width: 100%;
  }
  .spending-row {
    display: grid;
    grid-template-columns: 2rem minmax(0, 1fr) auto 2.25rem;
    align-items: center;
    gap: .5rem;
    padding-block: 1rem;
    border-bottom: 1px solid var(--bulma-border-weak);

    td {
      padding: 0;
      border: 0;
      min-width: 0;
    }
    .icon.is-medium {
      width: 2rem;
      height: 2rem;
    }
  }

  .spending-description {
    font-size: .85rem;
    overflow-wrap: anywhere;
  }
  .spending-metadata {
    margin-top: .3rem;

    > .level-left {
      display: flex;
      flex-direction: row;
      flex-wrap: wrap;
      justify-content: flex-start;
      align-items: baseline;
      gap: .15rem .3rem !important;
      font-size: .65rem;
      overflow-wrap: anywhere;
      color: var(--bulma-text-weak);
    }

    .spending-category {
      display: none;
    }
  }

  .spending-row .spending-amount {
    font-size: .9rem !important;
    white-space: nowrap;
    font-variant-numeric: tabular-nums;
  }

  .delete-spending {
    width: 2.25rem;
    height: 2.75rem;
  }
  .delete-modal .modal-card {
    max-height: calc(100dvh - 2rem);
    margin-inline: 1rem;
    width: calc(100% - 2rem);
  }
  .delete-spending-heading {
    grid-template-columns: 2rem minmax(0, 1fr);
  }
  .delete-spending-heading h3 {
    font-size: 1.1rem;
    overflow-wrap: anywhere;
  }
}
</style>
