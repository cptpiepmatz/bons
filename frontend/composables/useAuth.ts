import { readonly, ref } from "vue";

import type { UserResponse } from "../../bindings/UserResponse.ts";
import { Api, ApiError } from "../services/api.ts";
import { i18n } from "../services/i18n.ts";

const user = ref<UserResponse>();
const isLoggingIn = ref(false);
const loginError = ref<string>();
const isLoggingOut = ref(false);
const logoutError = ref<string>();

async function checkSession(): Promise<boolean> {
  loginError.value = undefined;
  try {
    user.value = await Api.getCurrentUser();
    return true;
  } catch (error) {
    user.value = undefined;
    if (!(error instanceof ApiError) || error.status !== 401) {
      loginError.value = i18n.global.t("auth.sessionError");
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
      ? i18n.global.t("auth.invalidKey")
      : i18n.global.t("auth.loginError");
    return false;
  } finally {
    isLoggingIn.value = false;
  }
}

async function signOut(): Promise<boolean> {
  if (isLoggingOut.value) return false;

  isLoggingOut.value = true;
  logoutError.value = undefined;
  try {
    await Api.logout();
    user.value = undefined;
    loginError.value = undefined;
    return true;
  } catch {
    logoutError.value = i18n.global.t("auth.logoutError");
    return false;
  } finally {
    isLoggingOut.value = false;
  }
}

export function useAuth() {
  return {
    user: readonly(user),
    isLoggingIn: readonly(isLoggingIn),
    loginError: readonly(loginError),
    isLoggingOut: readonly(isLoggingOut),
    logoutError: readonly(logoutError),
    checkSession,
    signIn,
    signOut,
  };
}
