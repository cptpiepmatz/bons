/// Spending categories with English and German display labels.
///
/// Consumed by tools/generate_categories.ts through the ts-rs binding.
///
/// @category groceries { "en": "Groceries", "de": "Lebensmittel" }
/// @category drugstore { "en": "Drugstore", "de": "Drogerie" }
/// @category dining { "en": "Dining", "de": "Essen gehen" }
/// @category selfcare { "en": "Selfcare", "de": "Wellness" }
/// @category fashion { "en": "Fashion", "de": "Mode" }
/// @category leisure { "en": "Leisure", "de": "Freizeit" }
/// @category gifts { "en": "Gifts", "de": "Geschenke" }
/// @category home { "en": "Home", "de": "Haushalt" }
/// @category transport { "en": "Transport", "de": "Mobilität" }
/// @category health { "en": "Health", "de": "Gesundheit" }
/// @category other { "en": "Other", "de": "Sonstiges" }
#[derive(
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::EnumString,
    ts_rs::TS,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[ts(export, rename_all = "snake_case")]
pub enum Category {
    Groceries,
    Drugstore,
    Dining,
    Selfcare,
    Fashion,
    Leisure,
    Gifts,
    Home,
    Transport,
    Health,
    Other,
}
