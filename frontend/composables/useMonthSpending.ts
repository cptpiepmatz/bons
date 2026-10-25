import { onScopeDispose, readonly, ref, shallowRef, watch } from "vue";

import type { Dayjs } from "dayjs";

import { getMonth } from "../models/month.ts";
import { Api } from "../services/api.ts";

import type { SpendingResponse } from "../../bindings/SpendingResponse.ts";

export function useMonthSpending(monthDate: () => Dayjs) {
  const spendingEntries = shallowRef<SpendingResponse[]>([]);
  const isLoading = ref(false);
  const error = ref<string>();
  let controller: AbortController | undefined;

  async function refresh(): Promise<void> {
    controller?.abort();
    const request = new AbortController();
    controller = request;
    const date = monthDate();
    isLoading.value = true;
    error.value = undefined;
    spendingEntries.value = [];

    try {
      const entries = await Api.getSpendingForMonth(
        date.year(),
        getMonth(date),
        request.signal,
      );
      if (!request.signal.aborted) spendingEntries.value = entries;
    } catch (cause) {
      if (!request.signal.aborted) {
        error.value = cause instanceof Error ? cause.message : String(cause);
      }
    } finally {
      if (!request.signal.aborted) isLoading.value = false;
    }
  }

  watch(() => monthDate().format("YYYY-MM"), refresh, { immediate: true });
  onScopeDispose(() => controller?.abort());

  return {
    spendingEntries: readonly(spendingEntries),
    isLoading: readonly(isLoading),
    error: readonly(error),
    refresh,
  };
}
