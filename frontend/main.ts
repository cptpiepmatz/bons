import "@fontsource-variable/inter";
import "dayjs/locale/de.js";

import "./main.scss";

import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import localizedFormat from "dayjs/plugin/localizedFormat";
import { createApp } from "vue";

import App from "./App.vue";
import { loadCurrency } from "./composables/useCurrency.ts";
import { router } from "./router.ts";

dayjs.extend(customParseFormat);
dayjs.extend(localizedFormat);
dayjs.locale("de");

loadCurrency().catch((e) => console.error(`could not load currency, ${e}`));
createApp(App).use(router).mount("#app");
