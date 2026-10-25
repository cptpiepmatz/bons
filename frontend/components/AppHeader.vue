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
  <div class="header">
    <div
      class="
        is-flex
        is-flex-direction-row
        is-justify-content-space-between
        m-1
      ">
      <a
        class="left has-text-current"
        href="/"
        @click.prevent="router.push('/')"
      >
        <AppLogo class="mr-1" />
        <div class="is-flex is-flex-direction-row is-align-items-flex-end">
          <p class="title mb-0">bons</p>
          <p class="ml-2">Eure Ausgaben. Alles im Blick.</p>
        </div>
      </a>
      <div class="right">
        <UserAvatar :user-name="userName" class="avatar mr-2" />
        <a class="level has-text-current" style="gap: 0">
          <span class="level-item">{{ userName }}</span>
          <span class="level-item icon"><PhCaretDown /></span>
        </a>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.header {
  width: 100%;
  display: flex;
  justify-content: center;

  div {
    width: 100%;
    max-width: var(--content-width);

    .left {
      width: 100%;
      display: flex;
      flex-direction: row;
      align-items: center;

      img {
        width: 3em;
        height: auto;
      }

      .title {
        font-size: 2em;
      }

      p:not(.title) {
        font-size: 1em;
      }
    }

    .right {
      display: flex;
      flex-direction: row;
      align-items: center;
      justify-content: flex-end;

      .avatar {
        width: 2.3em;
        height: auto;
      }
    }
  }
}
</style>
