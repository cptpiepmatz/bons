import dayjs, { type Dayjs } from "dayjs";
import { onMounted, onUnmounted } from "vue";

/** Notify when the app becomes visible on a different local calendar day. */
export function useDayChange(
  onDayChange: (date: Dayjs, previousDate: Dayjs) => void,
) {
  let lastDate = dayjs();

  function onVisibilityChange() {
    if (document.visibilityState !== "visible") return;

    const date = dayjs();
    const previousDate = lastDate;
    lastDate = date;

    if (!date.isSame(previousDate, "day")) onDayChange(date, previousDate);
  }

  onMounted(() =>
    document.addEventListener("visibilitychange", onVisibilityChange)
  );
  onUnmounted(() =>
    document.removeEventListener("visibilitychange", onVisibilityChange)
  );
}
