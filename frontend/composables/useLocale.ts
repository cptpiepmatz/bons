import { computed } from "vue";

import { i18n, type Locale } from "../services/i18n.ts";

const locale = computed(() => i18n.global.locale.value as Locale);
const formatLocale = computed(() => locale.value === "de" ? "de-DE" : "en-US");
const dateFormatter = computed(() =>
  new Intl.DateTimeFormat(formatLocale.value)
);
const monthFormatter = computed(() =>
  new Intl.DateTimeFormat(formatLocale.value, {
    month: "long",
    year: "numeric",
  })
);
const shortDateFormatter = computed(() =>
  new Intl.DateTimeFormat(formatLocale.value, {
    day: "numeric",
    month: "short",
  })
);

export function useLocale() {
  return {
    locale,
    formatLocale,
    dateFormatter,
    monthFormatter,
    shortDateFormatter,
  };
}
