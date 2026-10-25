<script setup lang="ts">
import {
  PhAcorn,
  PhCalendarDots,
  PhCaretDown,
  PhPencilSimple,
  PhPlusSquare,
} from "@phosphor-icons/vue";
import dayjs from "dayjs";
import { computed, ref } from "vue";

import CategoryIcon from "../../components/CategoryIcon.vue";
import UserAvatar from "../../components/UserAvatar.vue";
import { useCurrency } from "../../composables/useCurrency.ts";
import {
  categories,
  categoryDisplayNames,
} from "../../generated/categories.ts";
import { Money } from "../../models/money.ts";
import { suggestCategory } from "../../services/classifier.ts";

import type { Category } from "../../../bindings/Category.ts";

const spent = ref<Money>();
const description = ref<string>();
const selectedCategory = ref<Category>("other");
const spentAt = ref(dayjs());

const isSpentValid = ref(true);
const categoryMenu = ref<HTMLDivElement>();
const spentAtInputDate = ref<HTMLInputElement>();
const spentAtInputText = ref<HTMLInputElement>();

const entryIsValid = computed(() =>
  isSpentValid.value && !!spent.value && !!(description.value?.length)
);

const { currencySymbol, currencyFormatter } = useCurrency();
const spentPlaceholder = computed(() => {
  if (!currencyFormatter.value) return;
  return currencyFormatter
    .value
    .formatToParts(0)
    .filter(({ type }) => type != "currency")
    .map(({ value }) => value)
    .join("");
});

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
  categoryMenu.value?.hidePopover();
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
      <div class="block mb-2">
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
      <div class="block mb-2">
        <label class="label mb-0" for="spent">Betrag</label>
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
            <a class="button is-static">{{ currencySymbol ?? "&ensp;" }}</a>
          </p>
        </div>
      </div>
      <div class="block mb-2">
        <label class="label mb-0" for="description">Beschreibung</label>
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
      <div class="block mb-2">
        <label class="label mb-0" for="category">Kategorie</label>
        <div class="field has-addons">
          <div class="control is-expanded">
            <input type="hidden" name="category" :value="selectedCategory">
            <div class="dropdown is-block">
              <div class="dropdown-trigger">
                <button
                  id="category"
                  type="button"
                  class="button is-fullwidth"
                  aria-controls="category-dropdown"
                  popovertarget="category-dropdown"
                >
                  <span class="icon is-small">
                    <CategoryIcon :category="selectedCategory" />
                  </span>
                  <span
                    class="is-flex-grow-1 has-text-left">{{ categoryDisplayNames[selectedCategory].de }}</span>
                  <span class="icon is-small"><PhCaretDown /></span>
                </button>
              </div>
              <div
                id="category-dropdown"
                ref="categoryMenu"
                popover="auto"
                class="dropdown-content category-menu is-position-fixed is-overflow-y-auto"
              >
                <button
                  v-for="category in categories"
                  :key="category"
                  type="button"
                  class="dropdown-item is-flex is-align-items-center"
                  :class="{ 'is-active': selectedCategory === category }"
                  :aria-pressed="selectedCategory === category"
                  @click="selectCategory(category)"
                >
                  <span class="icon is-small mr-2">
                    <CategoryIcon :category="category" />
                  </span>
                  <span>{{ categoryDisplayNames[category].de }}</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="block mb-2">
        <label class="label mb-0" for="spent_at">Datum</label>
        <div class="field has-addons">
          <p class="control is-expanded has-icons-right">
            <input
              class="input is-position-absolute is-invisible"
              id="spent_at"
              type="date"
              ref="spentAtInputDate"
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
      <p class="icon-text is-size-7 mb-1">
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
#category {
  anchor-name: --category-select;
}

.category-menu {
  position-anchor: --category-select;
  position-area: bottom center;
  position-try-fallbacks: flip-block;
  position-try-order: most-height;
  width: anchor-size(width);
  /* Percentages resolve against the available area above or below the button. */
  max-height: 100%;
  margin: 0;
  border: 0;
  padding-inline: 0;
  overscroll-behavior-y: contain;
}
</style>
