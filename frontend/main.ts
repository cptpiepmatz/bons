import { createApp } from "vue";

import "@fontsource-variable/inter";
import App from "./App.vue";
import "./main.scss";
import { router } from "./router.ts";

createApp(App).use(router).mount("#app");
