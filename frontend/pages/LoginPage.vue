<script setup lang="ts">
import { PhEye } from "@phosphor-icons/vue";
</script>

<template>
  <div class="container" :style="{maxWidth: '20rem'}">
    <div class="card">
      <div class="card-content">
        <div>promo</div>
        <hr>
        <div>
          <p>Willkommen bei bons</p>
          <p>Füg deinen Zugangsschlüssel ein, dann geht’s los.</p>
          <div class="control has-icons-right">
            <input class="input" type="password" placeholder="Hier einfügen" />
            <span class="icon is-small is-right">
              <PhEye weight="bold" />
            </span>
          </div>
          <button class="button is-primary is-fullwidth">
            Los geht's
          </button>
          <p>Noch keinen Schlüssel? Frag bei der Person nach, die bons eingerichtet hat.</p>
        </div>
      </div>
    </div>
  </div>
</template>
