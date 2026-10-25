<script setup lang="ts">
import { computed, ref } from "vue";

import type { Dayjs } from "dayjs";

import { SpendingResponse } from "../../../bindings/SpendingResponse";
import { useCurrency } from "../../composables/useCurrency";
import { Money } from "../../models/money";

const { currencyFormatter } = useCurrency();

const { monthDate } = defineProps<{ monthDate: Dayjs }>();

const spendingEntries = ref<SpendingResponse[]>([]);
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
