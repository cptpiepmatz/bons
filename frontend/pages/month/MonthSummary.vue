<script setup lang="ts">
import { ref } from "vue";

import type { Dayjs } from "dayjs";

import { useCurrency } from "../../composables/useCurrency";
import { Money } from "../../models/money";

const { currencyFormatter } = useCurrency();

const { monthDate } = defineProps<{ monthDate: Dayjs }>();

const totalSpent = ref(Money.fromCents(12345));
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
        <div></div>
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
