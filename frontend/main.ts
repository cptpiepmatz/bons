import { createApp } from "vue";
import "@fontsource-variable/inter";
import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";
import App from "./App.vue";
import "./main.scss";
import { router } from "./router.ts";

dayjs.extend(customParseFormat);

console.log(dayjs("September", "MMMM", "en"));

createApp(App).use(router).mount("#app");
