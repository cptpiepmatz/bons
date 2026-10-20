import { expect } from "@std/expect";

import { Money } from "./money.ts";

Deno.test("Money preserves cents and formats exact decimal amounts", () => {
  const cases: [number, string][] = [
    [0, "0.00"],
    [1, "0.01"],
    [1234, "12.34"],
    [-1, "-0.01"],
    [-1234, "-12.34"],
    [Number.MAX_SAFE_INTEGER, "90071992547409.91"],
    [Number.MIN_SAFE_INTEGER, "-90071992547409.91"],
  ];
  for (const [cents, expected] of cases) {
    const money = Money.fromCents(cents);
    expect(money.toCents()).toBe(cents);
    expect(money.toString()).toBe(expected);
  }
});

Deno.test("Money rejects fractional and unsafe cents", () => {
  for (const cents of [0.5, NaN, Infinity, Number.MAX_SAFE_INTEGER + 1]) {
    expect(() => Money.fromCents(cents)).toThrow(RangeError);
  }
});
