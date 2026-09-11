use std::str::FromStr;

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use sha2::{Digest, Sha256};
use sqlx::{
    sqlite::SqliteConnectOptions,
    types::chrono::{DateTime, Utc},
    Connection, FromRow, SqliteConnection,
};

pub struct Db {
    conn: SqliteConnection,
}

#[derive(FromRow)]
pub struct User {
    id: i64,
    name: String,
    key_hash: Vec<u8>,
    is_admin: bool,
    enabled: bool,
    created_at: DateTime<Utc>,
}

pub struct CreatedUser {
    pub user: User,
    key: [u8; 32],
}

impl CreatedUser {
    pub fn key_as_str(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.key)
    }
}

pub struct SessionToken<'u> {
    user: &'u User,
    token: [u8; 32],
}

impl SessionToken<'_> {
    pub fn token_as_str(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.token)
    }
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

    pub async fn create_user(
        &mut self,
        name: impl AsRef<str>,
        is_admin: bool,
    ) -> sqlx::Result<CreatedUser> {
        let name = name.as_ref();
        let key = rand::random::<[u8; 32]>();
        let key_hash = Sha256::digest(key);

        let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (name, key_hash, is_admin, created_at)
                VALUES (?, ?, ?, ?)
                RETURNING id, name, key_hash as "key_hash!: Vec<u8>", is_admin, enabled, created_at as "created_at: _"
            "#,
            name,
            key_hash.as_slice(),
            is_admin,
            Utc::now()
        )
        .fetch_one(&mut self.conn)
        .await?;

        Ok(CreatedUser { user, key })
    }

    pub async fn create_session_token<'u>(
        &mut self,
        user: &'u User,
        expires_at: DateTime<Utc>,
    ) -> sqlx::Result<SessionToken<'u>> {
        let token = rand::random::<[u8; 32]>();
        let token_hash = BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(token));

        sqlx::query!(
            r#"
                INSERT INTO sessions (user_id, token_hash, created_at, expires_at)
                VALUES (?, ?, ?, ?)
            "#,
            user.id,
            token_hash,
            Utc::now(),
            expires_at
        )
        .execute(&mut self.conn)
        .await?;

        Ok(SessionToken { user, token })
    }
}
