<script setup lang="ts">
import {
  PhCalendarDots,
  PhCaretDown,
  PhPencilSimple,
  PhPlusSquare,
} from "@phosphor-icons/vue";
import dayjs, { type Dayjs } from "dayjs";
import { computed, ref, watch } from "vue";

import CategoryIcon from "../../components/CategoryIcon.vue";
import { useCreateSpending } from "../../composables/useCreateSpending.ts";
import { useCurrency } from "../../composables/useCurrency.ts";
import { useDropdown } from "../../composables/useDropdown.ts";
import {
  categories,
  categoryDisplayNames,
} from "../../generated/categories.ts";
import { Money } from "../../models/money.ts";
import { suggestCategory } from "../../services/classifier.ts";

import type { Category } from "../../../bindings/Category.ts";

const emit = defineEmits<{ created: [] }>();
const { create, isSaving, error } = useCreateSpending();

// entry values
const spent = ref<Money>();
const description = ref<string>();
const selectedCategory = ref<Category>("other");
const spentAt = ref(dayjs());

// bindings
const spentText = ref("");
const isSpentValid = ref(true);
const categoryDropdown = useDropdown();
const spentAtInputDate = ref<HTMLInputElement>();
const spentAtInputText = ref<HTMLInputElement>();

// derived state
const validEntry = computed(() => {
  if (!isSpentValid.value) return;
  if (!spent.value) return;
  if (!description.value?.trim()) return;
  if (!spentAt.value.isValid()) return;
  if (isSaving.value) return;
  return {
    isSpentValid: isSpentValid.value,
    spent: spent.value,
    description: description.value,
    spentAt: spentAt.value,
    isSaving: isSaving.value,
  };
});

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

// actions
function selectCategory(category: Category) {
  selectedCategory.value = category;
  categoryDropdown.close();
}

async function submit() {
  if (!validEntry.value) return;
  const created = await create({
    spent_cents: validEntry.value.spent.toCents(),
    description: validEntry.value.description.trim(),
    category: selectedCategory.value,
    spent_at: spentAt.value.unix(),
  });
  if (!created) return;

  spentText.value = "";
  spent.value = undefined;
  description.value = undefined;
  selectedCategory.value = "other";
  isSpentValid.value = true;
  emit("created");
}

// reactive elements
function onSpentInput(event: Event) {
  const element = event.target as HTMLInputElement;
  isSpentValid.value = element.validity.valid;

  if (!element.value) {
    spent.value = undefined;
    return;
  }

  try {
    const number = Number(element.value.replaceAll(",", "."));
    if (Number.isNaN(number)) return;
    spent.value = Money.fromCents(Math.round(number * 100));
  } catch {
    spent.value = undefined;
    isSpentValid.value = false;
  }
}

function onDescriptionInput(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  description.value = value;
  selectedCategory.value = suggestCategory(value)?.category ?? "other";
}
</script>

<template>
  <div class="card">
    <form class="card-content" @submit.prevent="submit">
      <fieldset :disabled="isSaving">
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
                v-model="spentText"
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
                :value="description ?? ''"
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
              <div
                :ref="categoryDropdown.element"
                class="dropdown is-block"
                :class="{ 'is-active': categoryDropdown.isActive.value }"
                @keydown.esc="categoryDropdown.close"
              >
                <div class="dropdown-trigger">
                  <button
                    id="category"
                    type="button"
                    class="button is-fullwidth"
                    aria-controls="category-dropdown"
                    :aria-expanded="categoryDropdown.isActive.value"
                    @click="categoryDropdown.toggle"
                  >
                    <span class="icon is-small">
                      <CategoryIcon :category="selectedCategory" />
                    </span>
                    <span class="is-flex-grow-1 has-text-left">
                      {{ categoryDisplayNames[selectedCategory].de }}
                    </span>
                    <span class="icon is-small"><PhCaretDown /></span>
                  </button>
                </div>
                <div
                  id="category-dropdown"
                  v-show="categoryDropdown.isActive.value"
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
                :value="spentAt.format('YYYY-MM-DD')"
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
        <p v-if="error" class="help is-danger mb-2" role="alert">
        Die Ausgabe konnte nicht gespeichert werden. {{ error }}
      </p>
        <button type="submit" class="button is-primary is-fullwidth"
          :class="{ 'is-loading': isSaving }"
          :disabled="!validEntry">
        Ausgaben hinzufügen
      </button>
      </fieldset>
    </form>
  </div>
</template>

<style scoped>
#category {
  anchor-name: --category-select;
}

.category-menu {
  z-index: var(--bulma-dropdown-content-z, 20);
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
