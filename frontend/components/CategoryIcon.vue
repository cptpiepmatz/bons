<script setup lang="ts">
import {
  PhArmchair,
  PhBowlFood,
  PhCactus,
  PhCheers,
  PhCheese,
  PhGift,
  PhHandSoap,
  PhMoped,
  PhPill,
  PhSock,
  PhSprayBottle,
} from "@phosphor-icons/vue";

import type { Component } from "vue";

import type { Category } from "../../bindings/Category.ts";

defineProps<{ category: Category }>();

const categoryIcons = {
  groceries: PhCheese,
  drugstore: PhSprayBottle,
  dining: PhBowlFood,
  selfcare: PhHandSoap,
  fashion: PhSock,
  leisure: PhCheers,
  gifts: PhGift,
  home: PhArmchair,
  transport: PhMoped,
  health: PhPill,
  other: PhCactus,
} satisfies Record<Category, Component>;
</script>

<template>
  <component :is="categoryIcons[category]" />
</template>
