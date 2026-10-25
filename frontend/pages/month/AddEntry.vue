<script setup lang="ts">
import {
  PhAcorn,
  PhCalendarDots,
  PhCaretDown,
  PhPencilSimple,
  PhPlusSquare,
} from "@phosphor-icons/vue";
import dayjs from "dayjs";
import { computed, effect, onMounted, onUnmounted, ref, watch } from "vue";

import CategoryIcon from "../../components/CategoryIcon.vue";
import UserAvatar from "../../components/UserAvatar.vue";
import { useCurrency } from "../../composables/useCurrency.ts";
import { categories } from "../../generated/categories.ts";
import { Money } from "../../models/money.ts";
import { suggestCategory } from "../../services/classifier.ts";

import type { Category } from "../../../bindings/Category.ts";

const spent = ref<Money>();
const description = ref<string>();
const selectedCategory = ref<Category>("other");
const spentAt = ref(dayjs());

const isSpentValid = ref(true);
const isCategoryOpen = ref(false);
const spentAtInputDate = ref<HTMLInputElement>();
const spentAtInputText = ref<HTMLInputElement>();

const entryIsValid = computed(() =>
  isSpentValid.value && !!spent.value && !!(description.value?.length)
);

const { currencySymbol, currencyFormat } = useCurrency();
const spentPlaceholder = computed(() => {
  if (!currencyFormat.value) return;
  return currencyFormat
    .value
    .formatToParts(0)
    .filter(({ type }) => type != "currency")
    .map(({ value }) => value)
    .join("");
});

const onDocumentClick = () => isCategoryOpen.value = false;
onMounted(() => document.addEventListener("click", onDocumentClick));
onUnmounted(() => document.removeEventListener("click", onDocumentClick));

function onSpentInput(event: Event) {
  const element = event.target as HTMLInputElement;
  isSpentValid.value = element.validity.valid;

  if (!element.value) {
    spent.value = undefined;
    return;
  }

  try {
    let number = Number(element.value.replaceAll(",", "."));
    if (Number.isNaN(number)) return;
    spent.value = Money.fromCents(number * 100);
  } catch {}
}

function selectCategory(category: Category) {
  selectedCategory.value = category;
  isCategoryOpen.value = false;
}

function onDescriptionInput(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  description.value = value;
  selectedCategory.value = suggestCategory(value)?.category ?? "other";
}
</script>

<template>
  <div class="card">
    <div class="card-content">
      <div class="block">
        <div class="is-flex is-flex-direction-row is-align-items-center">
          <span class="icon has-text-primary is-large">
            <PhPlusSquare weight="fill" size="2.5em" />
          </span>
          <div class="is-flex is-flex-direction-column">
            <span class="title is-size-5">Neue Ausgabe</span>
            <span class="subtitle is-size-7">Was habt ihr ausgegeben?</span>
          </div>
        </div>
      </div>
      <div class="block">
        <label class="label" for="spent">Betrag</label>
        <div class="field has-addons">
          <p class="control is-expanded">
            <input
              id="spent"
              class="input"
              :class="{ 'is-danger': !isSpentValid }"
              type="text"
              inputmode="decimal"
              :placeholder="spentPlaceholder"
              pattern="^\d+([,.]\d{0,2})?$"
              @input="onSpentInput"
            >
          </p>
          <p class="control">
            <a class="button is-static">{{ currencySymbol }}</a>
          </p>
        </div>
      </div>
      <div class="block">
        <label class="label" for="description">Beschreibung</label>
        <div class="field has-addons">
          <p class="control is-expanded">
            <input
              id="description"
              class="input"
              type="text"
              placeholder="z.B. Wochenmarkt"
              @input="onDescriptionInput"
            >
          </p>
        </div>
      </div>
      <div class="block">
        <label class="label" for="category">Kategorie</label>
        <div class="field has-addons">
          <div class="control is-expanded">
            <input type="hidden" name="category" :value="selectedCategory">
            <div
              class="dropdown category-dropdown"
              :class="{ 'is-active': isCategoryOpen }"
              @click.stop
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
        <label class="label" for="spent_at">Datum</label>
        <div class="field has-addons">
          <p class="control is-expanded has-icons-right">
            <input
              class="input"
              id="spent_at"
              type="date"
              ref="spentAtInputDate"
              style="position: absolute; visibility: hidden;"
              @change="spentAt = dayjs(spentAtInputDate!.value)"
              @blur="spentAtInputText?.blur"
            >
            <input
              class="input"
              type="text"
              ref="spentAtInputText"
              :value="spentAt.format('L')"
              readonly
              @click="spentAtInputDate?.showPicker()"
            >
            <span class="icon is-right">
              <PhCalendarDots />
            </span>
          </p>
        </div>
      </div>
      <p class="icon-text mb-1" style="font-size: 0.8em">
        <span class="icon">
          <PhPencilSimple weight="fill" />
        </span>
        <span>Eingetragen von dir</span>
      </p>
      <button class="button is-primary is-fullwidth" :disabled="!entryIsValid">
        Ausgaben hinzufügen
      </button>
    </div>
  </div>
</template>

<style scoped>
.category-dropdown,
.category-dropdown .dropdown-trigger,
.category-dropdown .dropdown-menu {
  width: 100%;
}

.label {
  margin-bottom: 0;
}

.block {
  --bulma-block-spacing: 0.6em;
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
