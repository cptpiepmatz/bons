export const sortOptions = ["newest", "oldest", "highest", "lowest"] as const;

export type SortOptions = typeof sortOptions[number];

export const sortOptionDisplayNames = {
  newest: { en: "Newest first", de: "Neueste zuerst" },
  oldest: { en: "Oldest first", de: "Älteste zuerst" },
  highest: { en: "Highest amount first", de: "Höchster Betrag zuerst" },
  lowest: { en: "Lowest amount first", de: "Niedrigster Betrag zuerst" },
} as const satisfies Record<SortOptions, Record<"en" | "de", string>>;
