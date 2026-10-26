import { createI18n } from "vue-i18n";

import de from "../locales/de.ts";
import en from "../locales/en.ts";

export type Locale = "en" | "de";

export function detectLocale(languages: readonly string[]): Locale {
  for (const language of languages) {
    const base = language.toLowerCase().split("-")[0];
    if (base === "en" || base === "de") return base;
  }
  return "en";
}

export const i18n = createI18n({
  legacy: false,
  locale: detectLocale(
    typeof navigator === "undefined" ? [] : navigator.languages,
  ),
  fallbackLocale: "en",
  messages: { en, de },
});
