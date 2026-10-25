import type { CurrencyFormatter } from "./currencyFormatter.ts";

/** A monetary amount stored as integer cents. */
export class Money {
  private constructor(private readonly cents: number) {}

  static fromCents(cents: number): Money {
    if (!Number.isSafeInteger(cents)) {
      throw new RangeError("Cents must be a safe integer");
    }
    return new Money(cents);
  }

  toCents(): number {
    return this.cents;
  }

  /** Formats the amount using the supplied currency formatter. */
  format(formatter: CurrencyFormatter): string {
    return formatter.format(Math.floor(this.cents / 100));
  }
}
