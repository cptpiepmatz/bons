<script setup lang="ts">
import { PhAcorn, PhCaretLeft, PhCaretRight } from "@phosphor-icons/vue";
import dayjs, { type Dayjs } from "dayjs";
import { computed, effect, ref } from "vue";
import { useRouter } from "vue-router";

import { Category } from "../../../bindings/Category";
import { SpendingResponse } from "../../../bindings/SpendingResponse";
import CategoryIcon from "../../components/CategoryIcon.vue";
import { useCurrency } from "../../composables/useCurrency";
import {
  categories,
  categoryDisplayNames,
  categoryRecord,
} from "../../generated/categories";
import { Money } from "../../models/money";

const { currencyFormatter } = useCurrency();
const router = useRouter();

const monthDateInput = ref<HTMLInputElement>();

const { monthDate } = defineProps<{ monthDate: Dayjs }>();

function navigateMonth(date: Dayjs) {
  return router.push({
    name: "month",
    params: {
      year: date.format("YYYY"),
      month: date.locale("en").format("MMMM").toLowerCase(),
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

const spendingEntries = ref<SpendingResponse[]>([
  {
    id: 1,
    spent_cents: 6849,
    description: "Wocheneinkauf bei REWE",
    category: "groceries",
    user_id: 1,
    spent_at: monthDate.startOf("month").add(2, "day").unix(),
    created_at: monthDate.startOf("month").add(2, "day").unix(),
  },
  {
    id: 2,
    spent_cents: 1875,
    description: "Drogerieartikel bei dm",
    category: "drugstore",
    user_id: 1,
    spent_at: monthDate.startOf("month").add(4, "day").unix(),
    created_at: monthDate.startOf("month").add(4, "day").unix(),
  },
  {
    id: 3,
    spent_cents: 4250,
    description: "Abendessen beim Italiener",
    category: "dining",
    user_id: 1,
    spent_at: monthDate.startOf("month").add(7, "day").unix(),
    created_at: monthDate.startOf("month").add(7, "day").unix(),
  },
  {
    id: 4,
    spent_cents: 320,
    description: "Fahrkarte für die Straßenbahn",
    category: "transport",
    user_id: 1,
    spent_at: monthDate.startOf("month").add(9, "day").unix(),
    created_at: monthDate.startOf("month").add(9, "day").unix(),
  },
  {
    id: 5,
    spent_cents: 2490,
    description: "Geburtstagsgeschenk für Mama",
    category: "gifts",
    user_id: 1,
    spent_at: monthDate.startOf("month").add(12, "day").unix(),
    created_at: monthDate.startOf("month").add(12, "day").unix(),
  },
]);
const totalSpent = computed(() =>
  spendingEntries
    .value
    .map(({ spent_cents }) => Money.fromCents(spent_cents))
    .reduce((sum, entry) => sum.add(entry), Money.ZERO)
);

const topCategories = computed(() => {
  const spendingPerCategory = categoryRecord((category) =>
    spendingEntries
      .value
      .filter((entry) => entry.category == category)
      .map(({ spent_cents }) => Money.fromCents(spent_cents))
      .reduce((sum, entry) => sum.add(entry), Money.ZERO)
  );

  return (Object
    .entries(spendingPerCategory) as [Category, Money][])
    .filter(([, entry]) => !entry.isZero())
    .sort(([, a], [, b]) => b.toCents() - a.toCents())
    .slice(0, 4);
});

effect(() => console.log(topCategories.value));
</script>

<template>
  <div class="card" style="height: 100%;">
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
            class="button is-small"
            type="month"
            ref="monthDateInput"
            :value="monthDate.format('YYYY-MM')"
            @change="selectMonth"
            style="visibility: hidden; width: 10rem; position: absolute;"
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
            {{ currencyFormatter ? totalSpent.format(currencyFormatter) : "&ensp;" }}
          </p>
        </div>
        <div>
          <p>Insgesamt ausgegeben</p>
        </div>
        <div>
          <p v-if="spendingEntries.length">
            {{ spendingEntries.length }} Ausgaben
          </p>
          <p v-else>
            keine Ausgaben
          </p>
        </div>
      </div>
    </div>
    <hr>
    <div class="card-content py-3">
      <p class="title is-6 mb-2">Nach Kategorien</p>
      <div class="fixed-grid">
        <div class="grid">
          <div
            v-for="[category, spent] in topCategories"
            class="category-summary py-2 px-4"
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
    <hr>
    <div class="card-content">
      ghi
    </div>
  </div>
</template>

<style scoped lang="scss">
.category-summary {
  border: var(--bulma-control-border-width) solid var(--bulma-border);
  border-radius: var(--bulma-radius);
}

hr {
  margin: 0;
}
</style>
