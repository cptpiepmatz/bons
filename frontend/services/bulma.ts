/** Resolve a Bulma color variable to #RRGGBBAA using the current theme. */
export function getBulmaColor(name: string, fallbackName?: string): string {
  const fallback = fallbackName
    ? `var(--bulma-${fallbackName}, transparent)`
    : "transparent";
  const probe = document.createElement("span");
  probe.hidden = true;
  probe.style.color = `var(--bulma-${name}, ${fallback})`;
  document.documentElement.appendChild(probe);

  let color: string;
  try {
    color = getComputedStyle(probe).color;
  } finally {
    probe.remove();
  }

  const channels = color.match(/[\d.]+/g);
  if (!channels) return "#00000000";

  const [red, green, blue, alpha = "1"] = channels;
  return "#" +
    [Number(red), Number(green), Number(blue), Math.round(Number(alpha) * 255)]
      .map((channel) => channel.toString(16).padStart(2, "0"))
      .join("");
}
