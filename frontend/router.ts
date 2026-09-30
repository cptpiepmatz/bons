import { createRouter, createWebHistory } from "vue-router";
import dayjs, { type Dayjs } from "dayjs";
import LoginPage from "./pages/LoginPage.vue";
import MonthPage from "./pages/MonthPage.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/login", component: LoginPage },
    {
      path: "/:year?/:month?",
      component: MonthPage,
      props: (route) => ({ monthDate: route.meta.monthDate as Dayjs }),
      beforeEnter: (to) => {
        const year = to.params.year;
        const month = to.params.month;

        if (!year && !month) {
          to.meta.monthDate = dayjs();
          return true;
        }

        if (typeof year !== "string" || typeof month !== "string") return "/";

        const capitalizedMonth = month.charAt(0).toUpperCase() +
          month.slice(1).toLowerCase();
        const date = dayjs(`${capitalizedMonth} ${year}`, "MMMM YYYY", "en")
          .locale(dayjs.locale());

        if (!date.isValid()) return "/";
        to.meta.monthDate = date;
        return true;
      },
    },
  ],
});
