<script setup lang="ts">
import { ref } from "vue";

import CategoryIcon from "../../components/CategoryIcon.vue";
import { categories } from "../../generated/categories.ts";
import { suggestCategory } from "../../services/classifier.ts";

const categoryInput = ref("Aldi");
const isClassifying = ref(false);

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
            <input id="spent" class="input" type="text"
              placeholder="Your email">
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
                placeholder="Your email">
            </p>
          </div>
        </div>
        <div class="block">
          <label for="category">Kategorie</label>
          <div class="field has-addons">
            <div class="control is-expanded has-icons-left">
              <div class="select is-fullwidth">
                <select name="category">
                  <option
                    v-for="(value, key) in categories"
                    :value="key"
                  >{{ value.de }}</option>
                </select>
              </div>
              <span class="icon is-small is-left">
                <CategoryIcon category="groceries" />
              </span>
            </div>
          </div>
        </div>
        <div class="block">
          <label for="spent_at">Datum</label>
          <div class="field has-addons">
            <p class="control is-expanded">
              <input id="spent_at" class="input" type="text"
                placeholder="Your email">
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
