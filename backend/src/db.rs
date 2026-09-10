use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, Connection, SqliteConnection};

pub struct Db {
    conn: SqliteConnection,
}

impl Db {
    pub async fn connect() -> sqlx::Result<Self> {
        let options = SqliteConnectOptions::from_str("sqlite://bons.sqlite")?
            .create_if_missing(true)
            .foreign_keys(true);
        let mut conn = SqliteConnection::connect_with(&options).await?;

        sqlx::migrate!("backend/migrations").run(&mut conn).await?;

        Ok(Db { conn })
    }
}
