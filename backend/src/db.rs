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
    is_admin: bool,
    enabled: bool,
    created_at: DateTime<Utc>,
}

pub struct AccessToken<'u> {
    user: &'u User,
    token: [u8; 32],
}

impl AccessToken<'_> {
    pub fn token_as_str(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.token)
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
    ) -> sqlx::Result<User> {
        let name = name.as_ref();

        sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (name, is_admin, created_at)
                VALUES (?, ?, ?)
                RETURNING id, name, is_admin, enabled, created_at as "created_at: _"
            "#,
            name,
            is_admin,
            Utc::now()
        )
        .fetch_one(&mut self.conn)
        .await
    }

    pub async fn create_access_token<'u>(
        &mut self,
        user: &'u User,
    ) -> sqlx::Result<AccessToken<'u>> {
        let token = rand::random::<[u8; 32]>();
        let token_hash = Sha256::digest(token);

        sqlx::query!(
            r#"
                INSERT INTO access_tokens (user_id, token_hash, created_at)
                VALUES (?, ?, ?)
            "#,
            user.id,
            token_hash.as_slice(),
            Utc::now()
        )
        .execute(&mut self.conn)
        .await?;

        Ok(AccessToken { user, token })
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
