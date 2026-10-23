<script setup lang="ts">
import { PhCaretDown } from "@phosphor-icons/vue";
import { ref } from "vue";

import CategoryIcon from "../../components/CategoryIcon.vue";
import { categories } from "../../generated/categories.ts";
import { suggestCategory } from "../../services/classifier.ts";

import type { Category } from "../../../bindings/Category.ts";

const dateLocaleOptions: Intl.DateTimeFormatOptions = {
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
};

const categoryInput = ref("Aldi");
const isClassifying = ref(false);
const selectedCategory = ref<Category>("groceries");
const isCategoryOpen = ref(false);
const spentAt = ref(new Date());
const spentAtInputDate = ref<HTMLInputElement | undefined>(undefined);
const spentAtInputText = ref<HTMLInputElement | undefined>(undefined);

function selectCategory(category: Category) {
  selectedCategory.value = category;
  isCategoryOpen.value = false;
}

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

<template>
  <div class="card" style="height: 100%;">
    <div class="card-content">
      <div class="block">
        <label for="spent">Betrag</label>
        <div class="field has-addons">
          <p class="control is-expanded">
            <input
              id="spent"
              class="input"
              type="text"
              inputmode="decimal"
              placeholder="0,00"
              pattern="^\d+([,.]\d{0,2})?$"
            >
          </p>
          <p class="control">
            <a class="button is-static">€</a>
          </p>
        </div>
        <div class="block">
          <label for="description">Beschreibung</label>
          <div class="field has-addons">
            <p class="control is-expanded">
              <input id="description" class="input" type="text"
                placeholder="z.B. Wochenmarkt">
            </p>
          </div>
        </div>
        <div class="block">
          <label for="category">Kategorie</label>
          <div class="field has-addons">
            <div class="control is-expanded">
              <input type="hidden" name="category" :value="selectedCategory">
              <div
                class="dropdown category-dropdown"
                :class="{ 'is-active': isCategoryOpen }"
                @keydown.esc.stop.prevent="isCategoryOpen = false"
              >
                <div class="dropdown-trigger">
                  <button
                    id="category"
                    type="button"
                    class="button is-fullwidth"
                    :aria-expanded="isCategoryOpen"
                    aria-controls="category-dropdown"
                    @click="isCategoryOpen = !isCategoryOpen"
                  >
                    <span class="icon is-small">
                      <CategoryIcon :category="selectedCategory" />
                    </span>
                    <span
                      class="category-label">{{ categories[selectedCategory].de }}</span>
                    <span class="icon is-small"><PhCaretDown /></span>
                  </button>
                </div>
                <div id="category-dropdown" class="dropdown-menu">
                  <div class="dropdown-content">
                    <button
                      v-for="(value, key) in categories"
                      :key="key"
                      type="button"
                      class="dropdown-item is-flex is-align-items-center"
                      :class="{ 'is-active': selectedCategory === key }"
                      :aria-pressed="selectedCategory === key"
                      @click="selectCategory(key)"
                    >
                      <span class="icon is-small mr-2">
                        <CategoryIcon :category="key" />
                      </span>
                      <span>{{ value.de }}</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="block">
          <label for="spent_at">Datum</label>
          <div class="field has-addons">
            <p class="control is-expanded">
              <input
                class="input"
                id="spent_at"
                type="date"
                ref="spentAtInputDate"
                style="position: absolute; visibility: hidden;"
                @change="spentAt = new Date(spentAtInputDate!.value)"
                @blur="spentAtInputText?.blur"
              >
              <input
                class="input"
                type="text"
                ref="spentAtInputText"
                :value="spentAt.toLocaleDateString('de-DE', dateLocaleOptions)"
                readonly
                @click="spentAtInputDate?.showPicker()"
              >
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.category-dropdown,
.category-dropdown .dropdown-trigger,
.category-dropdown .dropdown-menu {
  width: 100%;
}

.category-label {
  flex: 1;
  text-align: left;
}

.category-dropdown .dropdown-content {
  max-height: min(18rem, 30dvh);
  overflow-y: auto;
  overscroll-behavior-y: contain;
}
</style>
