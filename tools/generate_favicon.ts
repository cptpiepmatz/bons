import { initWasm, Resvg } from "@resvg/resvg-wasm";

// Resolve paths relative to this script, independent of the working directory.
const source = new URL("../logo/logo.svg", import.meta.url);
const output = new URL("../frontend/generated/icons/", import.meta.url);
const wasm = new URL(
  "index_bg.wasm",
  import.meta.resolve("@resvg/resvg-wasm"),
);
await initWasm(await Deno.readFile(wasm));
const svg = await Deno.readTextFile(source);

function renderPng(size: number, background?: string): Uint8Array {
  const renderer = new Resvg(svg, {
    fitTo: { mode: "width", value: size },
    background,
  });
  try {
    const image = renderer.render();
    try {
      return image.asPng();
    } finally {
      image.free();
    }
  } finally {
    renderer.free();
  }
}

// ICO stores a directory followed by each image. PNG payloads preserve alpha.
const sizes = [16, 32, 48];
const images = sizes.map((size) => renderPng(size));
const directorySize = 6 + 16 * images.length;
const ico = new Uint8Array(
  directorySize + images.reduce((total, image) => total + image.length, 0),
);
const header = new DataView(ico.buffer);
header.setUint16(2, 1, true); // Type: icon.
header.setUint16(4, images.length, true);
let offset = directorySize;
for (const [index, image] of images.entries()) {
  const entry = 6 + index * 16;
  header.setUint8(entry, sizes[index]);
  header.setUint8(entry + 1, sizes[index]);
  header.setUint16(entry + 4, 1, true); // Color planes.
  header.setUint16(entry + 6, 32, true); // Bits per pixel.
  header.setUint32(entry + 8, image.length, true);
  header.setUint32(entry + 12, offset, true);
  ico.set(image, offset);
  offset += image.length;
}

await Deno.mkdir(output, { recursive: true });
await Deno.writeTextFile(new URL("favicon.svg", output), svg);
await Deno.writeFile(new URL("favicon.ico", output), ico);
// Apple touch icons use an opaque background.
await Deno.writeFile(
  new URL("apple-touch-icon.png", output),
  renderPng(180, "white"),
);
console.log("Generated SVG, ICO (16/32/48px), and Apple touch icon (180px).");
