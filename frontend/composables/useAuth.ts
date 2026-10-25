import { readonly, ref } from "vue";

import { Api, ApiError } from "../services/api.ts";

import type { UserResponse } from "../../bindings/UserResponse.ts";

const user = ref<UserResponse>();
const isLoggingIn = ref(false);
const loginError = ref<string>();

async function checkSession(): Promise<boolean> {
  loginError.value = undefined;
  try {
    user.value = await Api.getCurrentUser();
    return true;
  } catch (error) {
    user.value = undefined;
    if (!(error instanceof ApiError) || error.status !== 401) {
      loginError.value =
        "Die Sitzung konnte nicht geprüft werden. Bitte prüf, ob das Backend läuft und aktuell ist, und versuch es erneut.";
    }
    return false;
  }
}

async function signIn(key: string): Promise<boolean> {
  if (isLoggingIn.value) return false;

  isLoggingIn.value = true;
  loginError.value = undefined;
  try {
    user.value = (await Api.login({ key: key.trim() })).user;
    return true;
  } catch (error) {
    loginError.value = error instanceof ApiError && error.status === 401
      ? "Der Zugangsschlüssel ist ungültig."
      : "Die Anmeldung ist gerade nicht möglich. Bitte versuch es erneut.";
    return false;
  } finally {
    isLoggingIn.value = false;
  }
}

export function useAuth() {
  return {
    user: readonly(user),
    isLoggingIn: readonly(isLoggingIn),
    loginError: readonly(loginError),
    checkSession,
    signIn,
  };
}
