import { onMounted, onUnmounted, Ref, ref } from "vue";

type MediaQueryMatches = {
  matches: boolean;
  media: string;
};

export function useMediaQuery(query: string): Ref<boolean> {
  const matchesRef = ref(false);
  let mediaQueryList: MediaQueryList;

  const update = ({ matches }: MediaQueryMatches) => matchesRef.value = matches;

  onMounted(() => {
    mediaQueryList = globalThis.matchMedia(query);
    mediaQueryList.addEventListener("change", update);
    update(mediaQueryList);
  });

  onUnmounted(() => mediaQueryList?.removeEventListener("change", update));

  return matchesRef;
}
