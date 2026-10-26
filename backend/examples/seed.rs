use std::{fs::OpenOptions, io::ErrorKind, path::Path};

use bons::{
    auth::Key,
    category::Category,
    db::{Db, DATABASE_PATH},
    money::Money,
};
use chrono::{DateTime, Datelike, Duration, Months, TimeZone, Utc};

static_toml::static_toml! {
    static SEED = include_toml!("backend/examples/seed.toml");
}

#[derive(Clone, Copy)]
enum Locale {
    English,
    German,
}

impl Locale {
    fn from_language(language: &str) -> Option<Self> {
        match language
            .split(['-', '_', '.', '@'])
            .next()?
            .to_ascii_lowercase()
            .as_str()
        {
            "en" => Some(Self::English),
            "de" => Some(Self::German),
            _ => None,
        }
    }

    fn from_env_or_os() -> Result<Self, Box<dyn std::error::Error>> {
        match std::env::var("BONS_SEED_LOCALE") {
            Ok(language) => Self::from_language(&language).ok_or_else(|| {
                format!(
                    "Unsupported BONS_SEED_LOCALE {language:?}; expected English (en) or German (de)."
                )
                .into()
            }),
            Err(std::env::VarError::NotPresent) => Ok(Self::detect()),
            Err(error) => Err(error.into()),
        }
    }

    fn detect() -> Self {
        // Use the first supported OS language, just as the frontend does for the browser.
        sys_locale::get_locales()
            .find_map(|locale| Self::from_language(&locale))
            .unwrap_or(Self::English)
    }

    fn language(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::German => "de",
        }
    }
}

fn month_entries(
    month: DateTime<Utc>,
    months_ago: usize,
    locale: Locale,
) -> Vec<(DateTime<Utc>, i64, &'static str, Category)> {
    let purchases = &SEED.purchases;
    let count = usize::try_from(SEED.month_entry_counts[months_ago])
        .expect("seed month entry counts must be non-negative");
    let days = (month.checked_add_months(Months::new(1)).unwrap() - month).num_days();

    (0..count)
        .map(|index| {
            let purchase = &purchases[(index * 5 + months_ago * 3) % purchases.len()];
            let description = match locale {
                Locale::English => purchase.en,
                Locale::German => purchase.de,
            };
            let category = purchase.category.parse().expect("invalid seed category");
            let day = index as i64 * days / count as i64;
            (
                month + Duration::days(day),
                purchase.cents + months_ago as i64 * 137 + (index / purchases.len()) as i64 * 79,
                description,
                category,
            )
        })
        .collect()
}

fn reserve_database(database: &Path) -> std::io::Result<()> {
    // Reserve the filename without overwriting an existing database.
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(database)
    {
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            return Err(std::io::Error::new(
                ErrorKind::AlreadyExists,
                format!(
                    "Database {} already exists; refusing to seed it. Remove the disposable database first.",
                    database.display()
                ),
            ));
        }
        Err(error) => return Err(error),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let locale = Locale::from_env_or_os()?;
    println!("Seed language: {}", locale.language());
    let database = DATABASE_PATH.as_path();
    reserve_database(database)?;

    let mut db = Db::connect_at(database).await?;
    let user = db
        .create_user_with_key(
            SEED.user.name,
            SEED.user.admin,
            Key::from_base64(SEED.user.access_key).expect("invalid seed access key"),
        )
        .await?
        .user;

    let now = Utc::now();
    let current_month = Utc
        .with_ymd_and_hms(now.year(), now.month(), 1, 12, 0, 0)
        .single()
        .unwrap();
    let mut total_entries = 0;
    for months_ago in 0..SEED.month_entry_counts.len() {
        let month = current_month
            .checked_sub_months(Months::new(months_ago as u32))
            .unwrap();
        let entries = month_entries(month, months_ago, locale);
        total_entries += entries.len();
        for (spent_at, cents, description, category) in entries {
            db.create_spending(
                Money::from_cents(cents),
                description.to_owned(),
                category,
                &user,
                spent_at,
            )
            .await?;
        }
        println!(
            "{}: {} spending entries",
            month.format("%Y-%m"),
            SEED.month_entry_counts[months_ago]
        );
    }

    println!(
        "Created {} with {total_entries} spending entries across {} months.",
        database.display(),
        SEED.month_entry_counts.len()
    );
    println!("Development access key: {}", SEED.user.access_key);
    println!("This is a public, development-only key. Never use it in production.");
    Ok(())
}
