import { createRouter, createWebHistory } from "vue-router";

import LoginPage from "./pages/LoginPage.vue";
import MonthPage from "./pages/MonthPage.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: MonthPage },
    { path: "/login", component: LoginPage },
  ],
});
