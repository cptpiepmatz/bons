<script setup lang="ts">
import { PhEye, PhEyeSlash } from "@phosphor-icons/vue";
import { ref } from "vue";

import AppLogo from "../components/AppLogo.vue";
import { useMediaQuery } from "../composables/useMediaQuery.ts";

const hideKey = ref(true);
const key = ref("");
const isMobile = useMediaQuery("(max-width: 768px)");
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
        <div>
          <div class="block">
            <p
              class="title mb-1 is-size-4 has-text-weight-bold">
              Willkommen bei bons
            </p>
            <p>Füg deinen Zugangsschlüssel ein, dann geht’s los.</p>
          </div>
          <div class="block">
            <div
              class="is-size-7 has-text-weight-bold mb-1"
            >
              Zugangsschlüssel
            </div>
            <div class="control has-icons-right">
              <input
                class="input" :type="hideKey ? 'password' : 'text'"
                placeholder="Hier einfügen" v-model="key"
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
          <div class="block">
            <button
              class="button is-primary is-fullwidth mb-2"
              :disabled="!key"
            >
            Los geht's
          </button>
            <p class="is-size-7">
              Noch keinen Schlüssel? Frag bei der Person nach, die bons eingerichtet hat.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
