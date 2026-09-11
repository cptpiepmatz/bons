#[derive(strum::AsRefStr, strum::EnumString, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
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
