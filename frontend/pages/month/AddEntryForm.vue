<script setup lang="ts">
import {
  PhCalendarDots,
  PhCaretDown,
  PhPencilSimple,
} from "@phosphor-icons/vue";
import dayjs from "dayjs";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { Category } from "../../../bindings/Category.ts";
import type { SpendingResponse } from "../../../bindings/SpendingResponse.ts";
import CategoryIcon from "../../components/CategoryIcon.vue";
import { useCreateSpending } from "../../composables/useCreateSpending.ts";
import { useCurrency } from "../../composables/useCurrency.ts";
import { useDropdown } from "../../composables/useDropdown.ts";
import { useLocale } from "../../composables/useLocale.ts";
import { useMediaQuery } from "../../composables/useMediaQuery.ts";
import {
  categories,
  categoryDisplayNames,
} from "../../generated/categories.ts";
import { Money } from "../../models/money.ts";
import { suggestCategory } from "../../services/classifier.ts";

const emit = defineEmits<{ created: [spending: SpendingResponse] }>();
const { t } = useI18n();
const { locale, dateFormatter } = useLocale();
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
const useNativeCategoryPicker = useMediaQuery(
  "(pointer: coarse) and (hover: none)",
);
watch(useNativeCategoryPicker, categoryDropdown.close);
const spentAtInputDate = ref<HTMLInputElement>();
const spentAtInputText = ref<HTMLInputElement>();

defineExpose({ isSaving, closeDropdown: categoryDropdown.close });

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

function onCategoryEscape(event: KeyboardEvent) {
  if (!categoryDropdown.isActive.value) return;
  event.preventDefault();
  event.stopPropagation();
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
  emit("created", created);
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
  <form @submit.prevent="submit">
    <fieldset :disabled="isSaving">
      <div class="block mb-2">
        <label class="label mb-0" for="spent">{{ t("common.amount") }}</label>
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
        <label class="label mb-0"
          for="description">{{ t("common.description") }}</label>
        <div class="field has-addons">
          <p class="control is-expanded">
            <input
              id="description"
              :value="description ?? ''"
              class="input"
              type="text"
              :placeholder="t('spending.descriptionPlaceholder')"
              @input="onDescriptionInput"
            >
          </p>
        </div>
      </div>
      <div class="block mb-2">
        <label class="label mb-0"
          for="category">{{ t("common.category") }}</label>
        <div class="field has-addons">
          <div class="control is-expanded">
            <div v-if="useNativeCategoryPicker" class="select is-fullwidth">
              <select id="category" v-model="selectedCategory" name="category">
                <option v-for="category in categories" :key="category"
                  :value="category">
                  {{ categoryDisplayNames[category][locale] }}
                </option>
              </select>
            </div>
            <div
              v-else
              :ref="categoryDropdown.element"
              class="dropdown is-block"
              :class="{ 'is-active': categoryDropdown.isActive.value }"
              @keydown.esc="onCategoryEscape"
            >
              <input type="hidden" name="category" :value="selectedCategory">
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
                      {{ categoryDisplayNames[selectedCategory][locale] }}
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
                  <span>{{ categoryDisplayNames[category][locale] }}</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="block mb-2">
        <label class="label mb-0" for="spent_at">{{ t("common.date") }}</label>
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
              :value="dateFormatter.format(spentAt.toDate())"
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
        <span>{{ t("spending.enteredByYou") }}</span>
      </p>
      <p v-if="error" class="help is-danger mb-2" role="alert">
        {{ t("spending.saveError") }} {{ error }}
      </p>
      <button type="submit" class="button is-primary is-fullwidth"
        :class="{ 'is-loading': isSaving }"
        :disabled="!validEntry">
        {{ t("spending.add") }}
      </button>
      <slot name="actions" />
    </fieldset>
  </form>
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
