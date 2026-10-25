import { computed, readonly, ref } from "vue";

import { CurrencyFormatter } from "../models/currencyFormatter.ts";
import { Api } from "../services/api.ts";

const currency = ref<string>();

const currencyFormatter = computed(() => {
  if (!currency.value) return;

  return new CurrencyFormatter("de-DE", currency.value);
});

const currencySymbol = computed(() => {
  if (!currencyFormatter.value) return;

  return currencyFormatter.value
    .formatToParts(0)
    .find((part) => part.type == "currency")?.value ?? currency.value;
});

export function useCurrency() {
  return { currency: readonly(currency), currencyFormatter, currencySymbol };
}

export async function loadCurrency(): Promise<void> {
  currency.value = await Api.getCurrency();
}
