import dayjs, { type Dayjs } from "dayjs";
import { createRouter, createWebHistory } from "vue-router";

import { useAuth } from "./composables/useAuth.ts";
import { isMonth } from "./models/month.ts";
import LoginPage from "./pages/LoginPage.vue";
import MonthPage from "./pages/month/MonthPage.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/login", name: "login", component: LoginPage },
    {
      path: "/:year?/:month?",
      name: "month",
      component: MonthPage,
      props: (route) => ({ monthDate: route.meta.monthDate as Dayjs }),
    },
  ],
});

router.beforeEach(async (to) => {
  const { checkSession } = useAuth();
  const isAuthenticated = await checkSession();

  if (to.name === "login") return isAuthenticated ? "/" : true;
  if (!isAuthenticated) return { name: "login" };

  if (to.name !== "month") return;

  const year = to.params.year;
  const month = to.params.month;

  if (!year && !month) {
    to.meta.monthDate = dayjs();
    return true;
  }

  if (typeof year !== "string" || typeof month !== "string") return "/";

  const monthName = month.toLowerCase();
  if (!isMonth(monthName)) return "/";

  const capitalizedMonth = monthName.charAt(0).toUpperCase() +
    monthName.slice(1);
  const date = dayjs(`${capitalizedMonth} ${year}`, "MMMM YYYY", "en")
    .locale(dayjs.locale());

  if (!date.isValid()) return "/";
  to.meta.monthDate = date;
  return true;
});
