<script setup lang="ts">
import { Avatar, Style } from "@dicebear/core";
import thumbs from "@dicebear/styles/thumbs.json" with { type: "json" };
import { PhCaretDown } from "@phosphor-icons/vue";
import { computed, effect, onMounted, ref, watch } from "vue";

import { useMediaQuery } from "../composables/useMediaQuery.ts";
import { getBulmaColor } from "../services/bulma.ts";
import AppLogo from "./AppLogo.vue";

const backgroundColor = ref("#00000000");
const prefersDark = useMediaQuery("(prefers-color-scheme: dark)");

function updateBackgroundColor() {
  backgroundColor.value = getBulmaColor("text-color", "text");
}

effect(() => console.log(backgroundColor.value));

onMounted(updateBackgroundColor);
watch(prefersDark, updateBackgroundColor, { flush: "post" });

const userName = ref("Tim");
const style = new Style(thumbs);
const avatar = computed(() =>
  new Avatar(style, {
    seed: userName.value,
    scale: 0.9,
    backgroundColor: [backgroundColor.value],
    shapeColor: ["#B8245A"],
  }).toDataUri()
);
</script>

<style lang="scss" scoped>
.header {
  width: 100%;
  display: flex;
  justify-content: center;

  div {
    width: 100%;
    max-width: var(--content-width);

    .left {
      display: flex;
      flex-direction: row;
      align-items: center;

      img {
        width: 3em;
        height: auto;
      }

      .title {
        font-size: 2em;
      }

      p:not(.title) {
        font-size: 1em;
      }
    }

    .right {
      display: flex;
      flex-direction: row;
      align-items: center;
      justify-content: flex-end;

      .avatar {
        width: 2.3em;
        height: auto;
        border-radius: 50%;
        object-fit: cover;
      }
    }
  }
}
</style>

<template>
  <div class="header">
    <div
      class="
        is-flex
        is-flex-direction-row
        is-justify-content-space-between
        m-1
      ">
      <div class="left">
        <AppLogo class="mr-1" />
        <div class="is-flex is-flex-direction-row is-align-items-flex-end">
          <p class="title mb-0">bons</p>
          <p class="ml-2">Eure Ausgaben. Alles im Blick.</p>
        </div>
      </div>
      <div class="right">
        <img :src="avatar" alt="Avatar" class="avatar mr-2">
        <a class="level has-text-current" style="gap: 0">
          <span class="level-item">{{ userName }}</span>
          <span class="level-item icon"><PhCaretDown /></span>
        </a>
      </div>
    </div>
  </div>
</template>
