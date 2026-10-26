import { readonly, ref } from "vue";

import { Api } from "../services/api.ts";

export function useDeleteSpending() {
  const isDeleting = ref(false);
  const error = ref<string>();

  function clearError() {
    error.value = undefined;
  }

  async function remove(id: number): Promise<boolean> {
    if (isDeleting.value) return false;
    isDeleting.value = true;
    clearError();
    try {
      await Api.deleteSpending(id);
      return true;
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
      return false;
    } finally {
      isDeleting.value = false;
    }
  }

  return {
    remove,
    clearError,
    isDeleting: readonly(isDeleting),
    error: readonly(error),
  };
}
