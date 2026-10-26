<script setup lang="ts">
import { PhPlusSquare, PhX } from "@phosphor-icons/vue";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import AddEntryForm from "./AddEntryForm.vue";

const { modal = false } = defineProps<{ modal?: boolean }>();
const { t } = useI18n();
const emit = defineEmits<{ created: [] }>();
const dialog = ref<HTMLDialogElement>();
const form = ref<InstanceType<typeof AddEntryForm>>();
const isSaving = computed(() => form.value?.isSaving ?? false);

function open() {
  if (modal) dialog.value?.showModal();
}

function close() {
  if (isSaving.value) return;
  form.value?.closeDropdown();
  dialog.value?.close();
}

function onDialogClick(event: MouseEvent) {
  if (event.target !== dialog.value || !dialog.value) return;
  const bounds = dialog.value.getBoundingClientRect();
  if (
    event.clientX < bounds.left || event.clientX > bounds.right ||
    event.clientY < bounds.top || event.clientY > bounds.bottom
  ) close();
}

function onCreated() {
  close();
  emit("created");
}

watch(() => modal, close);
defineExpose({ open });
</script>

<template>
  <div v-show="!modal" class="card" @scroll="form?.closeDropdown()">
    <Teleport :to="dialog ?? 'body'" :disabled="!modal || !dialog">
      <div class="card-content">
        <header class="entry-heading block mb-2">
          <span v-if="!modal" class="icon has-text-primary is-large">
            <PhPlusSquare weight="fill" size="2.5em" />
          </span>
          <div class="is-flex is-flex-direction-column">
            <h2 id="add-entry-title"
              class="title is-size-5">{{ t("spending.new") }}</h2>
            <span class="subtitle is-size-7">{{ t("spending.question") }}</span>
          </div>
          <button v-if="modal" type="button" class="close-entry"
            :aria-label="t('common.close')" :disabled="isSaving"
            @click="close"><PhX size="1.4em" /></button>
        </header>
        <AddEntryForm ref="form" @created="onCreated">
          <template v-if="modal" #actions>
            <button type="button" class="button is-ghost is-fullwidth mt-2"
              :disabled="isSaving"
              @click="close">{{ t("common.cancel") }}</button>
          </template>
        </AddEntryForm>
      </div>
    </Teleport>
  </div>
  <dialog ref="dialog" class="card entry-dialog"
    aria-labelledby="add-entry-title" @cancel.prevent="close"
    @click="onDialogClick" @scroll="form?.closeDropdown()" />
</template>

<style scoped>
.entry-heading {
  display: flex;
  align-items: center;
}

.close-entry {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  align-self: flex-start;
  margin-left: auto;
  width: 2rem;
  height: 2rem;
  border: 0;
  border-radius: .5rem;
  background: transparent;
  color: var(--bulma-text);
  cursor: pointer;
}

.entry-dialog {
  width: calc(100% - 2rem);
  max-width: 28rem;
  max-height: calc(100dvh - 2rem);
  margin: auto;
  padding: 0;
  border: 1px solid var(--bulma-border-weak);
  border-radius: var(--bulma-card-radius);
  background: var(--bulma-scheme-main);
  color: var(--bulma-text);
  overflow-y: auto;
  overscroll-behavior: contain;

  &:not([open]) {
    display: none;
  }
  &::backdrop {
    background: rgb(0 0 0 / 55%);
  }
}

:global(html:has(.entry-dialog[open])) {
  overflow: hidden;
}
</style>
