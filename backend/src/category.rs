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
    EatingOut,
    Hairdresser,
    Household,
    Transport,
    Health,
    Entertainment,
    Clothing,
    Gifts,
    Other,
}
