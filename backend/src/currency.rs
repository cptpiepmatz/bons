include!(concat!(env!("OUT_DIR"), "/currency.rs"));

#[cfg(test)]
mod tests {
    use super::{Currency, ParseCurrencyError};

    #[test]
    fn known_currencies_round_trip() {
        for (code, expected) in [("EUR", Currency::EUR), ("USD", Currency::USD)] {
            let currency: Currency = code.parse().unwrap();
            assert_eq!(currency, expected);
            assert_eq!(currency.to_string(), code);
        }
    }

    #[test]
    fn unknown_currencies_are_preserved() {
        let currency: Currency = "ZZZ".parse().unwrap();
        assert_eq!(currency, Currency::Unknown("ZZZ".to_owned()));
        assert_eq!(currency.to_string(), "ZZZ");
    }

    #[test]
    fn blank_currencies_are_rejected() {
        for code in ["", " ", "\t\n"] {
            assert_eq!(code.parse::<Currency>(), Err(ParseCurrencyError));
        }
    }

    #[test]
    fn parse_error_implements_error_and_has_a_helpful_message() {
        let error: &dyn std::error::Error = &ParseCurrencyError;
        assert_eq!(
            error.to_string(),
            "currency code must not be blank (for example, EUR)"
        );
    }
}
