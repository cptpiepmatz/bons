use std::{env, fs, path::PathBuf, process::Command, sync::LazyLock};

use quote::{format_ident, quote};

static OUT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by cargo")));

fn main() {
    println!("cargo:rerun-if-changed=backend/build.rs");
    known_currencies();
}

fn known_currencies() {
    const JS_RUNTIME: &'static str = "deno";
    const JS_CODE: &'static str =
        r#"console.log(JSON.stringify(Intl.supportedValuesOf("currency")))"#;

    let output = Command::new(JS_RUNTIME)
        .args(["eval", JS_CODE])
        .output()
        .unwrap_or_else(|error| {
            panic!("Deno is required to generate currency codes; ensure deno is on PATH: {error}")
        });

    assert!(
        output.status.success(),
        "Deno failed to list supported currencies: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let codes: Vec<String> = serde_json::from_slice(&output.stdout)
        .expect("Deno must return a JSON array of currency codes");
    assert!(!codes.is_empty(), "Deno returned no currency codes");

    let variants: Vec<_> = codes
        .iter()
        .map(|code| {
            assert!(
                code.len() == 3 && code.bytes().all(|byte| byte.is_ascii_uppercase()),
                "Deno returned an invalid currency code: {code:?}"
            );
            format_ident!("{code}")
        })
        .collect();

    let generated = quote! {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum Currency {
            #(#variants,)*
            Unknown(String),
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ParseCurrencyError;

        impl std::fmt::Display for ParseCurrencyError {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "currency code must not be blank (for example, EUR)"
                )
            }
        }

        impl std::error::Error for ParseCurrencyError {}

        impl std::str::FromStr for Currency {
            type Err = ParseCurrencyError;

            fn from_str(code: &str) -> Result<Self, Self::Err> {
                if code.trim().is_empty() {
                    return Err(ParseCurrencyError);
                }
                Ok(match code {
                    #(#codes => Self::#variants,)*
                    _ => Self::Unknown(code.to_owned()),
                })
            }
        }

        impl std::fmt::Display for Currency {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(match self {
                    #(Self::#variants => #codes,)*
                    Self::Unknown(code) => code,
                })
            }
        }
    };

    let path = OUT_DIR.join("currency.rs");
    fs::write(path, generated.to_string()).expect("Failed to write generated currency enum");
}
