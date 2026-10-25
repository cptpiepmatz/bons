<script setup lang="ts">
import { PhEye, PhEyeSlash } from "@phosphor-icons/vue";
import { ref } from "vue";
import { useRouter } from "vue-router";

import AppLogo from "../components/AppLogo.vue";
import { useAuth } from "../composables/useAuth.ts";
import { useMediaQuery } from "../composables/useMediaQuery.ts";

const hideKey = ref(true);
const key = ref("");
const isMobile = useMediaQuery("(max-width: 768px)");
const router = useRouter();
const { signIn, isLoggingIn, loginError } = useAuth();

async function submit() {
  if (!key.value.trim()) return;
  if (await signIn(key.value)) {
    key.value = "";
    await router.replace("/");
  }
}
</script>

<style lang="scss" scoped>
.layout {
  height: 100%;
  width: 100%;

  > * {
    max-width: 30rem;
  }
}

.eye {
  color: var(--bulma-input-icon-color) !important;

  > :hover {
    color: var(--bulma-input-icon-hover-color) !important;
  }
}
</style>

<template>
  <div class="layout is-flex is-align-items-center is-justify-content-center">
    <div :class="isMobile ? 'm-4' : 'card'">
      <div :class="isMobile ? '' : 'card-content'">
        <div>
          <div class="is-flex is-flex-direction-column is-align-items-center">
            <div class="is-flex is-align-items-center">
              <AppLogo width="100em" class="mr-1" />
              <p class="title" :style="{fontSize: '4em'}">bons</p>
            </div>
            <p>Eure Ausgaben. Alles im Blick.</p>
          </div>
        </div>
        <hr>
        <form @submit.prevent="submit">
          <div class="block">
            <p
              class="title mb-1 is-size-4 has-text-weight-bold">
              Willkommen bei bons
            </p>
            <p>Füg deinen Zugangsschlüssel ein, dann geht’s los.</p>
          </div>
          <div class="block">
            <label
              for="access-key"
              class="is-size-7 has-text-weight-bold mb-1"
            >
              Zugangsschlüssel
            </label>
            <div class="control has-icons-right">
              <input
                id="access-key"
                class="input" :type="hideKey ? 'password' : 'text'"
                placeholder="Hier einfügen" v-model="key"
                :disabled="isLoggingIn"
                :aria-invalid="loginError ? true : undefined"
                :aria-describedby="loginError ? 'login-error' : undefined"
                @keydown.space.prevent
              />
              <span
                class="icon is-small is-right eye"
                @click="hideKey = !hideKey"
              >
                <PhEye v-if="hideKey" weight="bold" class="is-clickable" />
                <PhEyeSlash v-else weight="bold" class="is-clickable" />
              </span>
            </div>
          </div>
          <p v-if="loginError" id="login-error" class="help is-danger mb-3"
            role="alert">
            {{ loginError }}
          </p>
          <div class="block">
            <button
              type="submit"
              class="button is-primary is-fullwidth mb-2"
              :class="{ 'is-loading': isLoggingIn }"
              :disabled="!key.trim() || isLoggingIn"
            >
              Los geht's
            </button>
            <p class="is-size-7">
              Noch keinen Schlüssel? Frag bei der Person nach, die bons eingerichtet hat.
            </p>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
