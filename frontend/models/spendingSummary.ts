import { categoryRecord } from "../generated/categories.ts";

import type { Category } from "../../bindings/Category.ts";
import type { SpendingResponse } from "../../bindings/SpendingResponse.ts";

import { Money } from "./money.ts";

export function summarizeSpending(
  entries: readonly Pick<SpendingResponse, "category" | "spent_cents">[],
): { total: Money; categories: [Category, Money][] } {
  const totals = categoryRecord(() => Money.ZERO);
  let total = Money.ZERO;
  for (const entry of entries) {
    const amount = Money.fromCents(entry.spent_cents);
    total = total.add(amount);
    totals[entry.category] = totals[entry.category].add(amount);
  }
  const categories = (Object.entries(totals) as [Category, Money][])
    .filter(([, amount]) => !amount.isZero())
    .sort(([, a], [, b]) => b.toCents() - a.toCents())
    .slice(0, 4);
  return { total, categories };
}
