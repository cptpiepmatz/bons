<script setup lang="ts">
import { PhEye, PhEyeSlash } from "@phosphor-icons/vue";
import Logo from "../components/Logo.vue";
import { ref } from "vue";

const hideKey = ref(true);
const key = ref("");
</script>

<style lang="scss" scoped>
.layout {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  width: 100vw;
  background-color: var(--bulma-white-ter);

  @media (prefers-color-scheme: dark) {
    background-color: var(--bulma-black-ter);
  }

  * {
    max-width: 25rem;
  }
}

.eye {
  color: var(--bulma-input-icon-color) !important;

  > * {
    pointer-events: auto;
    cursor: pointer;
  }

  > :hover {
    color: var(--bulma-input-icon-hover-color) !important;
  }
}
</style>

<template>
  <div class="layout">
    <div class="card">
      <div class="card-content">
        <div>
          <div class="is-flex is-flex-direction-column is-align-items-center">
            <div class="is-flex is-align-items-center">
              <Logo width="100em" />
              <p class="title" :style="{fontSize: '4em'}">bons</p>
            </div>
            <p>Eure Ausgaben. Alles im Blick.</p>
          </div>
        </div>
        <hr>
        <div>
          <div class="block">
            <p
              class="title mb-1 is-size-4 has-text-weight-bold">Willkommen bei bons</p>
            <p>Füg deinen Zugangsschlüssel ein, dann geht’s los.</p>
          </div>
          <div class="block">
            <div :style="{fontSize: '0.8em', fontWeight: 'bold'}"
              class="mb-1">Zugangsschlüssel</div>
            <div class="control has-icons-right">
              <input class="input" :type="hideKey ? 'password' : 'text'"
                placeholder="Hier einfügen" v-model="key"
                @keydown.space.prevent />
              <span class="icon is-small is-right eye"
                @click="hideKey = !hideKey">
                <PhEye v-if="hideKey" weight="bold" />
                <PhEyeSlash v-else weight="bold" />
              </span>
            </div>
          </div>
          <div class="block">
            <button class="button is-primary is-fullwidth mb-2"
              :disabled="!key">
            Los geht's
          </button>
            <p
              :style="{fontSize: '0.7em'}">Noch keinen Schlüssel? Frag bei der Person nach, die bons eingerichtet hat.</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
