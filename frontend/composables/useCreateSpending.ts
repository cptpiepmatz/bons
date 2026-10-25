import { readonly, ref } from "vue";

import { Api } from "../services/api.ts";

import type { CreateSpendingRequest } from "../../bindings/CreateSpendingRequest.ts";
import type { SpendingResponse } from "../../bindings/SpendingResponse.ts";

export function useCreateSpending() {
  const isSaving = ref(false);
  const error = ref<string>();

  async function create(
    spending: CreateSpendingRequest,
  ): Promise<SpendingResponse | undefined> {
    if (isSaving.value) return;
    isSaving.value = true;
    error.value = undefined;
    try {
      return await Api.createSpending(spending);
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
    } finally {
      isSaving.value = false;
    }
  }

  return { create, isSaving: readonly(isSaving), error: readonly(error) };
}
