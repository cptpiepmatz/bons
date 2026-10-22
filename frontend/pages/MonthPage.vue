<script setup lang="ts">
import { Avatar, Style } from "@dicebear/core";
import thumbs from "@dicebear/styles/thumbs.json" with { type: "json" };
import { PhCaretDown } from "@phosphor-icons/vue";
import { ref } from "vue";

import type { Dayjs } from "dayjs";

import Logo from "../components/Logo.vue";
import { suggestCategory } from "../services/classifier.ts";

const { monthDate } = defineProps<{ monthDate: Dayjs }>();
const formattedMonth = monthDate.format("MMMM YYYY");
const categoryInput = ref("Aldi");
const isClassifying = ref(false);

const style = new Style(thumbs);
const avatar = new Avatar(style, {
  seed: "Tim",
  backgroundColor: ["16161a"],
  shapeColor: ["#B8245A"],
}).toDataUri();

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
.avatar {
  width: 10rem;
  height: 10rem;
  border-radius: 50%;
  object-fit: cover;
}

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
    max-width: 700px;
  }
}
</style>

<template>
  <div class="layout">
    <div class="header">
      <div
        class="is-flex is-flex-direction-row is-justify-content-space-between">
        <left class="is-flex is-flex-direction-row is-align-items-center">
          <Logo width="100em" class="mr-1" />
          <p class="title" :style="{fontSize: '4em'}">bons</p>
          <p>Eure Ausgaben. Alles im Blick.</p>
        </left>
        <right class="is-flex is-flex-direction-row is-align-items-center">
          <img :src="avatar" alt="Avatar" class="avatar">
          <p>Tim</p>
          <span class="icon">
            <PhCaretDown />
          </span>
        </right>
      </div>
    </div>
    <hr>
    <div></div>
  </div>
</template>
