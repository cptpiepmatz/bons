import type { CurrencyFormatter } from "./currencyFormatter.ts";

/** A monetary amount stored as integer cents. */
export class Money {
  private constructor(private readonly cents: number) {}

  static readonly ZERO = new Money(0);

  static fromCents(cents: number): Money {
    if (!Number.isSafeInteger(cents)) {
      throw new RangeError("Cents must be a safe integer");
    }
    return new Money(cents);
  }

  toCents(): number {
    return this.cents;
  }

  add(other: number | Money): Money {
    const otherMoney = other instanceof Money ? other : Money.fromCents(other);
    return new Money(this.cents + otherMoney.cents);
  }

  /** Formats the amount using the supplied currency formatter. */
  format(formatter: CurrencyFormatter): string {
    return formatter.format(this.cents / 100);
  }

  isZero(): boolean {
    return this.cents == 0;
  }
}
