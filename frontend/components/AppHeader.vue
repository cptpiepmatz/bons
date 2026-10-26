<script setup lang="ts">
import { PhCaretDown, PhSignOut } from "@phosphor-icons/vue";
import { computed } from "vue";
import { useRouter } from "vue-router";

import { useAuth } from "../composables/useAuth.ts";
import { useDropdown } from "../composables/useDropdown.ts";
import AppLogo from "./AppLogo.vue";
import UserAvatar from "./UserAvatar.vue";

const router = useRouter();

const { user, signOut, isLoggingOut, logoutError } = useAuth();
const userName = computed(() => user.value?.name ?? "");
const userDropdown = useDropdown();

async function logout() {
  if (!await signOut()) return;
  userDropdown.close();
  await router.push({ name: "login" });
}
</script>

<template>
  <div class="is-flex is-justify-content-center">
    <div
      class="
        is-flex
        is-justify-content-space-between
        content-width
        m-1
      ">
      <a
        class="is-flex is-align-items-center is-flex-grow-1 has-text-current"
        href="/"
        @click.prevent="router.push('/')"
      >
        <AppLogo class="logo mr-1" />
        <div class="is-flex is-flex-direction-row is-align-items-flex-end">
          <p class="title is-size-3 mb-0">bons</p>
          <p class="ml-2">Eure Ausgaben. Alles im Blick.</p>
        </div>
      </a>
      <div
        :ref="userDropdown.element"
        class="dropdown is-right is-flex is-align-items-center is-justify-content-flex-end"
        :class="{ 'is-active': userDropdown.isActive.value }"
      >
        <UserAvatar :user-name="userName" class="avatar mr-2" />
        <div class="dropdown-trigger">
          <button
            type="button"
            class="level mb-0 is-gap-1 has-text-current"
            aria-controls="user-dropdown"
            :aria-expanded="userDropdown.isActive.value"
            @click="userDropdown.toggle"
          >
            <span>{{ userName }}</span>
            <span class="icon"><PhCaretDown /></span>
          </button>
        </div>
        <div id="user-dropdown" class="dropdown-menu">
          <div class="dropdown-content">
            <button
              type="button"
              class="dropdown-item"
              :disabled="isLoggingOut"
              @click="logout"
            >
              <span class="icon-text">
                <span class="icon">
                  <PhSignOut />
                </span>
                <span>Abmelden</span>
              </span>
            </button>
            <p v-if="logoutError" class="help is-danger px-4" role="alert">
              {{ logoutError }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.logo {
  width: 3em;
  height: auto;
}

.avatar {
  width: 2.3em;
  height: auto;
}
</style>
