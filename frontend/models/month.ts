import type { Dayjs } from "dayjs";

export const months = [
  "january",
  "february",
  "march",
  "april",
  "may",
  "june",
  "july",
  "august",
  "september",
  "october",
  "november",
  "december",
] as const;

export type Month = typeof months[number];

export function isMonth(value: string): value is Month {
  return months.some((month) => month === value);
}

export function getMonth(date: Dayjs): Month {
  if (!date.isValid()) {
    throw new RangeError("Cannot get month from invalid date");
  }
  return months[date.month()];
}
