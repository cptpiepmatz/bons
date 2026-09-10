use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, Connection, SqliteConnection};

pub struct Db {
    conn: SqliteConnection,
}

impl Db {
    pub async fn connect() -> sqlx::Result<Self> {
        let options =
            SqliteConnectOptions::from_str("sqlite://bons.sqlite")?.create_if_missing(true);
        let conn = SqliteConnection::connect_with(&options).await?;
        let db = Db { conn };
        db.init().await?;
        Ok(db)
    }

    async fn init(&self) -> sqlx::Result<()> {
        sqlx::query_file!("backend/sql/init.sql");
        todo!()
    }
}
