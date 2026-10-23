<script setup lang="ts">
import { Avatar, Style } from "@dicebear/core";
import thumbs from "@dicebear/styles/thumbs.json" with { type: "json" };
import { computed, onMounted, ref, watch } from "vue";

import { useMediaQuery } from "../composables/useMediaQuery.ts";
import { getBulmaColor } from "../services/bulma.ts";

const props = defineProps<{ userName: string }>();

const backgroundColor = ref("#00000000");
const prefersDark = useMediaQuery("(prefers-color-scheme: dark)");

function updateBackgroundColor() {
  backgroundColor.value = getBulmaColor("text-color", "text");
}

onMounted(updateBackgroundColor);
watch(prefersDark, updateBackgroundColor, { flush: "post" });

const style = new Style(thumbs);
const avatar = computed(() =>
  new Avatar(style, {
    seed: props.userName,
    scale: 0.9,
    backgroundColor: [backgroundColor.value],
    shapeColor: ["#B8245A"],
  }).toDataUri()
);
</script>

<style lang="scss" scoped>
img {
  border-radius: 50%;
  object-fit: cover;
}
</style>

<template>
  <img :src="avatar" alt="Avatar">
</template>
