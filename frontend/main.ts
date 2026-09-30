import { createApp } from "vue";
import "@fontsource-variable/inter";
import dayjs from "dayjs";
import "dayjs/locale/de.js";
import customParseFormat from "dayjs/plugin/customParseFormat";
import App from "./App.vue";
import "./main.scss";
import { router } from "./router.ts";

dayjs.extend(customParseFormat);
dayjs.locale("de");

console.log(dayjs("September", "MMMM", "en"));

createApp(App).use(router).mount("#app");
