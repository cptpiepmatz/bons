use std::{fs::OpenOptions, io::ErrorKind, path::Path};

use bons::{
    auth::Key,
    category::Category,
    db::{Db, DATABASE_PATH},
    money::Money,
};
use chrono::{DateTime, Datelike, Duration, Months, TimeZone, Utc};

const DEV_KEY: &str = "bons2667-all-your-receipts-are-belong-to-us";
// Current month first: deliberately mix short and long spending lists.
const MONTH_ENTRY_COUNTS: [usize; 6] = [18, 7, 26, 12, 34, 9];

fn month_entries(
    month: DateTime<Utc>,
    months_ago: usize,
) -> Vec<(DateTime<Utc>, i64, &'static str, Category)> {
    let purchases = [
        (8649, "Wocheneinkauf bei REWE", Category::Groceries),
        (1275, "Drogerieartikel bei dm", Category::Drugstore),
        (320, "Fahrkarte für die Straßenbahn", Category::Transport),
        (4250, "Abendessen beim Italiener", Category::Dining),
        (2990, "Geburtstagsgeschenk", Category::Gifts),
        (1800, "Kino", Category::Leisure),
        (3850, "Haarschnitt", Category::Selfcare),
        (5990, "Neue Schuhe", Category::Fashion),
        (2475, "Pflanzen für den Balkon", Category::Home),
        (1490, "Apotheke", Category::Health),
        (950, "Porto und Versand", Category::Other),
        (4832, "Einkauf bei ALDI", Category::Groceries),
        (890, "Kaffee und Kuchen", Category::Dining),
        (2195, "Haushaltsartikel bei Rossmann", Category::Drugstore),
        (5800, "Bahnfahrt am Wochenende", Category::Transport),
        (2500, "Museum und Ausstellung", Category::Leisure),
        (6500, "Massage", Category::Selfcare),
        (3499, "Pullover", Category::Fashion),
        (7990, "Regal fürs Wohnzimmer", Category::Home),
        (2800, "Physiotherapie", Category::Health),
        (1990, "Blumen als Mitbringsel", Category::Gifts),
        (1200, "Schlüsseldienst: Ersatzschlüssel", Category::Other),
        (2367, "Obst und Gemüse vom Wochenmarkt", Category::Groceries),
        (1670, "Mittagessen im Bistro", Category::Dining),
    ];
    let count = MONTH_ENTRY_COUNTS[months_ago];
    let days = (month.checked_add_months(Months::new(1)).unwrap() - month).num_days();

    (0..count)
        .map(|index| {
            let (cents, description, category) =
                purchases[(index * 5 + months_ago * 3) % purchases.len()];
            let day = index as i64 * days / count as i64;
            (
                month + Duration::days(day),
                cents + months_ago as i64 * 137 + (index / purchases.len()) as i64 * 79,
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
    let database = DATABASE_PATH.as_path();
    reserve_database(database)?;

    let mut db = Db::connect_at(database).await?;
    let user = db
        .create_user_with_key("Development", true, Key::from_base64(DEV_KEY).unwrap())
        .await?
        .user;

    let now = Utc::now();
    let current_month = Utc
        .with_ymd_and_hms(now.year(), now.month(), 1, 12, 0, 0)
        .single()
        .unwrap();
    let mut total_entries = 0;
    for months_ago in 0..MONTH_ENTRY_COUNTS.len() {
        let month = current_month
            .checked_sub_months(Months::new(months_ago as u32))
            .unwrap();
        let entries = month_entries(month, months_ago);
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
            MONTH_ENTRY_COUNTS[months_ago]
        );
    }

    println!(
        "Created {} with {total_entries} spending entries across {} months.",
        database.display(),
        MONTH_ENTRY_COUNTS.len()
    );
    println!("Development access key: {DEV_KEY}");
    println!("This is a public, development-only key. Never use it in production.");
    Ok(())
}
