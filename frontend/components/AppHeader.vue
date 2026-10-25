<script setup lang="ts">
import { PhCaretDown } from "@phosphor-icons/vue";
import { ref } from "vue";
import { useRouter } from "vue-router";

import AppLogo from "./AppLogo.vue";
import UserAvatar from "./UserAvatar.vue";

const router = useRouter();

const userName = ref("Tim");
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
      <div class="is-flex is-align-items-center is-justify-content-flex-end">
        <UserAvatar :user-name="userName" class="avatar mr-2" />
        <a class="is-flex is-align-items-center has-text-current">
          <span>{{ userName }}</span>
          <span class="icon"><PhCaretDown /></span>
        </a>
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
