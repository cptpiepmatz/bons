import { expect } from "@std/expect";

import { suggestCategory } from "./classifier.ts";

Deno.test("recognizes merchant names and explains exact matches", () => {
  expect(suggestCategory("Aldi")).toEqual({
    input: "Aldi",
    category: "groceries",
    match: "exact",
    alias: "Aldi",
    distance: 0,
  });
  for (
    const [input, expected] of [
      ["Edeka", "groceries"],
      ["Rewe", "groceries"],
      ["dm", "drugstore"],
      ["Rossmann", "drugstore"],
      ["IKEA", "home"],
      ["H&M", "fashion"],
      ["Deutsche Bahn", "transport"],
    ]
  ) {
    expect(suggestCategory(input)?.category, input).toBe(expected);
  }
});

Deno.test("normalizes case, accents, whitespace and punctuation", () => {
  for (
    const [input, expected] of [
      ["  ALDI   Süd, Berlin #123  ", "groceries"],
      ["MÜLLER", "drugstore"],
      ["Mueller", "drugstore"],
      ["Müller".normalize("NFD"), "drugstore"],
      ["dm-drogerie", "drugstore"],
      ["C & A", "fashion"],
      ["Europa-Park", "leisure"],
    ]
  ) {
    expect(suggestCategory(input)?.category, input).toBe(expected);
    expect(suggestCategory(input)?.match, input).toBe("exact");
  }
});

Deno.test("matches German and English services as whole words", () => {
  for (
    const [input, expected] of [
      ["Friseur", "selfcare"],
      ["Massage", "selfcare"],
      ["Pediküre", "selfcare"],
      ["Kino", "leisure"],
      ["Girls night out", "dining"],
      ["Café", "dining"],
      ["Apotheke", "health"],
      ["medizinische Fußpflege", "health"],
      ["Parkgebühren", "transport"],
      ["Schuhe", "fashion"],
      ["Möbel", "home"],
    ]
  ) {
    expect(suggestCategory(input)?.category, input).toBe(expected);
    expect(suggestCategory(input)?.match, input).toBe("keyword");
  }
});

Deno.test("explicit gifts override merchant and service matches", () => {
  for (
    const input of [
      "Zara Geburtstagsgeschenk",
      "Geschenk von Aldi",
      "cinema gift",
    ]
  ) {
    expect(suggestCategory(input)?.category, input).toBe("gifts");
  }
  expect(suggestCategory("Zara")?.category).toBe("fashion");
});

Deno.test("handles merchant substitutions, insertions, deletions and swapped letters", () => {
  for (
    const [input, expected] of [
      ["Aldu", "groceries"],
      ["Aldii", "groceries"],
      ["Edea", "groceries"],
      ["Edeak", "groceries"],
      ["Adli Süd Berlin", "groceries"],
      ["Rossman", "drugstore"],
      ["Rassman", "drugstore"],
      ["Deutcshe Bahm Berlin", "transport"],
    ]
  ) {
    expect(suggestCategory(input)?.category, input).toBe(expected);
    expect(suggestCategory(input)?.match, input).toBe("fuzzy");
  }
  expect(suggestCategory("Edeak")?.distance).toBe(1);
  expect(suggestCategory("Rassman")?.distance).toBe(2);
});

Deno.test("short aliases require exact whole-word matches", () => {
  for (
    const input of [
      "admin",
      "admission",
      "d",
      "dn",
      "dmm",
      "dbb",
      "shellfish",
      "barber",
    ]
  ) {
    expect(suggestCategory(input), input).toBeUndefined();
  }
});

Deno.test("abstains for unknown, empty or ambiguous input", () => {
  for (
    const input of [
      "",
      "  ",
      "!!!",
      "Unknown merchant",
      "Amazon",
      "Aldi Rossmann",
      "Kino Friseur",
      "Aldu Rossman",
    ]
  ) {
    expect(suggestCategory(input), input).toBeUndefined();
  }
});

Deno.test("exact matches take precedence over fuzzy matches", () => {
  expect(suggestCategory("Aldi Rossman")?.category).toBe("groceries");
  expect(suggestCategory("Aldi Rossman")?.match).toBe("exact");
});
