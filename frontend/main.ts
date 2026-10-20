import "@fontsource-variable/inter";
import "dayjs/locale/de.js";

import "./main.scss";

import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import { createApp } from "vue";

import App from "./App.vue";
import { router } from "./router.ts";

dayjs.extend(customParseFormat);
dayjs.locale("de");

console.log(dayjs("September", "MMMM", "en"));

createApp(App).use(router).mount("#app");
