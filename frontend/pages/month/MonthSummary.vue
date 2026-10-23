<script setup lang="ts">
import type { Dayjs } from "dayjs";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const formattedMonth = monthDate.format("MMMM YYYY");
</script>

<template>
  <div class="card" style="height: 100%;">
    <div class="card-content"></div>
    <div class="card-content"></div>
    <div class="card-content"></div>
  </div>
</template>
