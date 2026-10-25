/** A number formatter that always formats monetary values. */
export class CurrencyFormatter extends Intl.NumberFormat {
  declare private readonly currencyFormatterBrand: void;

  constructor(
    locales: Intl.LocalesArgument,
    currency: string,
    options?: Omit<Intl.NumberFormatOptions, "style" | "currency">,
  ) {
    super(locales, { ...options, style: "currency", currency });
  }
}
