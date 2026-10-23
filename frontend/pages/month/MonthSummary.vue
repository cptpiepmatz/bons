<script setup lang="ts">
import type { Dayjs } from "dayjs";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const formattedMonth = monthDate.format("MMMM YYYY");
</script>

<template>
  <div></div>
</template>
