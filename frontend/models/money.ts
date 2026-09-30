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

  /** Returns a decimal amount with two digits, without a currency symbol. */
  toString(): string {
    const absoluteCents = Math.abs(this.cents);
    const whole = Math.floor(absoluteCents / 100);
    const fraction = String(absoluteCents % 100).padStart(2, "0");
    return `${this.cents < 0 ? "-" : ""}${whole}.${fraction}`;
  }
}
