use std::{fs::OpenOptions, io::ErrorKind, path::PathBuf};

use bons::{auth::Key, category::Category, db::Db, money::Money};
use chrono::{Datelike, Duration, Months, TimeZone, Utc};
use clap::Parser;

/// Seed a missing development database. Never use this key in production.
#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "bons.sqlite")]
    database: PathBuf,
}

const DEV_KEY: &str = "bons2667-all-your-receipts-are-belong-to-us";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Reserve the filename without overwriting an existing database.
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args.database)
    {
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            println!(
                "Database {} already exists; skipping development seed and leaving it unchanged.",
                args.database.display()
            );
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    }

    let mut db = Db::connect_at(&args.database).await?;
    let user = db
        .create_user_with_key("Development", true, Key::from_base64(DEV_KEY).unwrap())
        .await?
        .user;

    let now = Utc::now();
    let current_month = Utc
        .with_ymd_and_hms(now.year(), now.month(), 1, 12, 0, 0)
        .single()
        .unwrap();
    let previous_month = current_month.checked_sub_months(Months::new(1)).unwrap();
    let entries = [
        (1, 8649, "Wocheneinkauf bei REWE", Category::Groceries),
        (3, 1275, "Drogerieartikel bei dm", Category::Drugstore),
        (7, 320, "Fahrkarte für die Straßenbahn", Category::Transport),
        (12, 4250, "Abendessen beim Italiener", Category::Dining),
        (16, 2990, "Geburtstagsgeschenk", Category::Gifts),
        (20, 1800, "Kino", Category::Leisure),
    ];

    for month in [previous_month, current_month] {
        for (day, cents, description, category) in entries {
            db.create_spending(
                Money::from_cents(cents),
                description.to_owned(),
                category,
                &user,
                month + Duration::days(day - 1),
            )
            .await?;
        }
    }

    println!(
        "Created {} with 12 spending entries.",
        args.database.display()
    );
    println!(
        "Months: {} and {}",
        previous_month.format("%Y-%m"),
        current_month.format("%Y-%m")
    );
    println!("Development access key: {DEV_KEY}");
    println!("This is a public, development-only key. Never use it in production.");
    Ok(())
}
