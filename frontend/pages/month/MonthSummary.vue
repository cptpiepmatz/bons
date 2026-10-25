<script setup lang="ts">
import { computed, effect, ref } from "vue";

import type { Dayjs } from "dayjs";

import { SpendingResponse } from "../../../bindings/SpendingResponse";
import { useCurrency } from "../../composables/useCurrency";
import { Money } from "../../models/money";

const { currencyFormatter } = useCurrency();

const { monthDate } = defineProps<{ monthDate: Dayjs }>();

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
  Money.fromCents(
    spendingEntries.value.reduce(
      (acc, { spent_cents }) => acc + spent_cents,
      0,
    ),
  )
);
</script>

<template>
  <div class="card" style="height: 100%;">
    <div class="card-content">
      <div>
        <div>{{ monthDate.format("MMMM YYYY") }}</div>
        <div>{{ currencyFormatter && totalSpent.format(currencyFormatter) }}</div>
        <div>
          <p>Insgesamt ausgegeben</p>
        </div>
        <div>{{ spendingEntries.length }} Ausgaben</div>
      </div>
    </div>
    <hr>
    <div class="card-content">
      def
    </div>
    <hr>
    <div class="card-content">
      ghi
    </div>
  </div>
</template>
