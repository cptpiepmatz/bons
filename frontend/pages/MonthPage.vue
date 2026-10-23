<script setup lang="ts">
import { Avatar, Style } from "@dicebear/core";
import thumbs from "@dicebear/styles/thumbs.json" with { type: "json" };
import { PhCaretDown } from "@phosphor-icons/vue";
import { computed, effect, onMounted, ref, watch } from "vue";

import type { Dayjs } from "dayjs";

import Logo from "../components/Logo.vue";
import { useMediaQuery } from "../composables/useMediaQuery.ts";
import { getBulmaColor } from "../services/bulma.ts";
import { suggestCategory } from "../services/classifier.ts";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const formattedMonth = monthDate.format("MMMM YYYY");
const categoryInput = ref("Aldi");
const isClassifying = ref(false);
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

async function testSuggestCategory() {
  const text = categoryInput.value.trim();
  if (!text || isClassifying.value) return;

  isClassifying.value = true;
  console.time("Category suggestion");
  try {
    const result = await suggestCategory(text);
    if (result === undefined) {
      console.info("No unambiguous category match.");
    } else {
      console.log(result);
    }
  } catch (error) {
    console.error(error);
  } finally {
    console.timeEnd("Category suggestion");
    isClassifying.value = false;
  }
}
</script>

<style lang="scss" scoped>
$content-width: 900px;

.layout {
  height: 100%;
  width: 100%;
}

.header {
  width: 100%;
  display: flex;
  justify-content: center;

  div {
    width: 100%;
    max-width: $content-width;

    left {
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

    right {
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
  <div class="layout">
    <div class="header">
      <div
        class="
          is-flex 
          is-flex-direction-row 
          is-justify-content-space-between
          m-1
        ">
        <left class="is-flex is-flex-direction-row is-align-items-center">
          <Logo class="mr-1" />
          <div class="is-flex is-flex-direction-row is-align-items-flex-end">
            <p class="title mb-0">bons</p>
            <p class="ml-2">Eure Ausgaben. Alles im Blick.</p>
          </div>
        </left>
        <right class="is-flex is-flex-direction-row is-align-items-center">
          <img :src="avatar" alt="Avatar" class="avatar mr-2">
          <a class="level has-text-current" style="gap: 0">
            <span class="level-item">{{ userName }}</span>
            <span class="level-item icon"><PhCaretDown /></span>
          </a>
        </right>
      </div>
    </div>
    <hr class="mt-0">
    <div></div>
  </div>
</template>
