import { onMounted, onUnmounted, ref } from "vue";

export function useDropdown() {
  const element = ref<HTMLElement>();
  const isActive = ref(false);

  function close() {
    isActive.value = false;
  }

  function toggle() {
    isActive.value = !isActive.value;
  }

  function onOutsideClick(event: MouseEvent) {
    if (
      event.target instanceof Node && !element.value?.contains(event.target)
    ) {
      close();
    }
  }

  onMounted(() => document.addEventListener("click", onOutsideClick));
  onUnmounted(() => document.removeEventListener("click", onOutsideClick));

  return { element, isActive, toggle, close };
}
