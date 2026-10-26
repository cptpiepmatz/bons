import type { Category } from "../../bindings/Category.ts";
import { categoryAliases } from "./categoryAliases.ts";

export type CategorySuggestion = {
  input: string;
  category: Category;
  match: "exact" | "keyword" | "fuzzy";
  alias: string;
  distance: number;
};

function normalize(text: string): string {
  return text.toLowerCase()
    .normalize("NFKD")
    .replace(/\p{M}/gu, "")
    .replace(/ß/g, "ss")
    .replace(/[^\p{L}\p{N}]+/gu, " ")
    .trim()
    .replace(/\s+/g, " ");
}

type Alias = { category: Category; alias: string; normalized: string };

const merchants: Alias[] = [];
const keywords: Alias[] = [];
for (const category of Object.keys(categoryAliases) as Category[]) {
  for (const alias of categoryAliases[category].merchants) {
    merchants.push({ category, alias, normalized: normalize(alias) });
  }
  for (const alias of categoryAliases[category].keywords) {
    keywords.push({ category, alias, normalized: normalize(alias) });
  }
}

function wholeWordMatches(input: string, aliases: Alias[]): Alias[] {
  return aliases.filter(({ normalized }) =>
    ` ${input} `.includes(` ${normalized} `)
  );
}

// Restricted Damerau–Levenshtein (optimal string alignment): also counts an
// adjacent letter swap as one edit. Merchant names are short, so a matrix is cheap.
function editDistance(left: string, right: string): number {
  const matrix = Array.from(
    { length: left.length + 1 },
    (_, row) =>
      Array.from(
        { length: right.length + 1 },
        (_, column) => row === 0 ? column : column === 0 ? row : 0,
      ),
  );
  for (let row = 1; row <= left.length; row++) {
    for (let column = 1; column <= right.length; column++) {
      matrix[row][column] = Math.min(
        matrix[row - 1][column] + 1,
        matrix[row][column - 1] + 1,
        matrix[row - 1][column - 1] +
          (left[row - 1] === right[column - 1] ? 0 : 1),
      );
      if (
        row > 1 && column > 1 && left[row - 1] === right[column - 2] &&
        left[row - 2] === right[column - 1]
      ) {
        matrix[row][column] = Math.min(
          matrix[row][column],
          matrix[row - 2][column - 2] + 1,
        );
      }
    }
  }
  return matrix[left.length][right.length];
}

function pickUniqueCategory(
  input: string,
  matches: Alias[],
  match: CategorySuggestion["match"],
  distance = 0,
): CategorySuggestion | undefined {
  if (
    !matches.length ||
    new Set(matches.map((alias) => alias.category)).size !== 1
  ) return undefined;
  const { category, alias } = matches[0];
  return { input, category, match, alias, distance };
}

// Unknown or conflicting matches stay unclassified. This is a suggestion, not an
// automatic assignment: callers can let the user confirm especially fuzzy matches.
export function suggestCategory(text: string): CategorySuggestion | undefined {
  const input = text.trim();
  const normalized = normalize(input);
  if (!normalized) return undefined;

  const keywordMatches = wholeWordMatches(normalized, keywords);
  const gifts = keywordMatches.filter(({ category }) => category === "gifts");
  if (gifts.length) return pickUniqueCategory(input, gifts, "keyword");

  const exact = wholeWordMatches(normalized, merchants);
  if (exact.length) return pickUniqueCategory(input, exact, "exact");
  if (keywordMatches.length) {
    return pickUniqueCategory(input, keywordMatches, "keyword");
  }

  const tokens = normalized.split(" ");
  let bestDistance = Infinity;
  let best: Alias[] = [];
  for (const merchant of merchants) {
    const name = merchant.normalized;
    const length = name.replaceAll(" ", "").length;
    if (length < 4) continue; // Never guess short names such as dm, DB, or UCI.
    const maxDistance = length < 8 ? 1 : 2;
    const wordCount = name.split(" ").length;
    for (let start = 0; start + wordCount <= tokens.length; start++) {
      const candidate = tokens.slice(start, start + wordCount).join(" ");
      if (candidate.replaceAll(" ", "").length < 4) continue;
      if (Math.abs(candidate.length - name.length) > maxDistance) continue;
      const distance = editDistance(candidate, name);
      if (distance > maxDistance || distance > bestDistance) continue;
      if (distance < bestDistance) {
        bestDistance = distance;
        best = [];
      }
      best.push(merchant);
    }
  }
  return pickUniqueCategory(input, best, "fuzzy", bestDistance);
}
