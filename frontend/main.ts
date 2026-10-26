import "@fontsource-variable/inter";
import "dayjs/locale/de.js";

import "./main.scss";

import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import localizedFormat from "dayjs/plugin/localizedFormat";
import { createApp, watchEffect } from "vue";

import App from "./App.vue";
import { loadCurrency } from "./composables/useCurrency.ts";
import { router } from "./router.ts";
import { detectLocale, i18n } from "./services/i18n.ts";

dayjs.extend(customParseFormat);
dayjs.extend(localizedFormat);
watchEffect(() => {
  const locale = i18n.global.locale.value;
  dayjs.locale(locale);
  document.documentElement.lang = locale;
  const description = document.querySelector<HTMLMetaElement>(
    'meta[name="description"]',
  );
  if (description) description.content = i18n.global.t("meta.description");
});

globalThis.addEventListener("languagechange", () => {
  i18n.global.locale.value = detectLocale(navigator.languages);
});

loadCurrency().catch((e) => console.error(`could not load currency, ${e}`));
createApp(App).use(i18n).use(router).mount("#app");
