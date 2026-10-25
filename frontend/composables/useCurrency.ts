import { computed, readonly, ref } from "vue";

const currency = ref<string>();

const currencyFormat = computed(() => {
  if (!currency.value) return;

  return new Intl.NumberFormat("de-DE", {
    style: "currency",
    currency: currency.value,
  });
});

const currencySymbol = computed(() => {
  if (!currencyFormat.value) return;

  return currencyFormat.value
    .formatToParts(0)
    .find((part) => part.type == "currency")?.value ?? currency.value;
});

export function useCurrency() {
  return { currency: readonly(currency), currencyFormat, currencySymbol };
}

export async function loadCurrency(): Promise<void> {
  const response = await fetch("/api/currency");
  if (!response.ok) {
    throw new Error(`Failed to load currency: ${response.status}`);
  }

  currency.value = await response.text();
}
