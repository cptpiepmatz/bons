use std::str::FromStr;

use chrono::{DateTime, Duration, Month, TimeZone, Utc};
use sqlx::{sqlite::SqliteConnectOptions, Connection, SqliteConnection};

use crate::{
    auth::{Key, SessionToken},
    money::Money,
    Category,
};

pub struct Db {
    conn: SqliteConnection,
}

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
    key: Key,
}

impl CreatedUser {
    pub fn key_as_str(&self) -> String {
        self.key.as_base64()
    }
}

pub struct CreatedSessionToken<'u> {
    user: &'u User,
    token: SessionToken,
}

impl CreatedSessionToken<'_> {
    pub fn token_as_str(&self) -> String {
        self.token.as_base64()
    }
}

pub struct Spending {
    id: i64,
    spent: Money,
    description: String,
    category: Category,
    user_id: i64,
    spent_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
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
        let key = Key::random();
        let key_hash = key.hash();
        let created_at = Utc::now();

        let created_user = sqlx::query!(
            r#"
                INSERT INTO users (name, key_hash, is_admin, created_at)
                VALUES (?, ?, ?, ?)
                RETURNING id, enabled
            "#,
            name,
            key_hash.as_slice(),
            is_admin,
            created_at.timestamp()
        )
        .fetch_one(&mut self.conn)
        .await?;

        let user = User {
            id: created_user.id,
            name: name.to_owned(),
            key_hash,
            is_admin,
            enabled: created_user.enabled,
            created_at,
        };

        Ok(CreatedUser { user, key })
    }

    pub async fn verify_login(&mut self, key: &Key) -> sqlx::Result<Option<User>> {
        let key_hash = key.hash();

        let user = sqlx::query!(
            r#"
                SELECT id, name, key_hash, is_admin, enabled, created_at
                FROM users
                WHERE key_hash = ? AND enabled
            "#,
            key_hash.as_slice()
        )
        .fetch_optional(&mut self.conn)
        .await?;

        let Some(user) = user else {
            return Ok(None);
        };
        let created_at = DateTime::from_timestamp(user.created_at, 0)
            .ok_or_else(|| sqlx::Error::Protocol("invalid created_at timestamp".into()))?;

        Ok(Some(User {
            id: user.id,
            name: user.name,
            key_hash: user.key_hash,
            is_admin: user.is_admin,
            enabled: user.enabled,
            created_at,
        }))
    }

    pub async fn create_session_token<'u>(
        &mut self,
        user: &'u User,
        expires_at: DateTime<Utc>,
    ) -> sqlx::Result<CreatedSessionToken<'u>> {
        let token = SessionToken::random();
        let token_hash = token.hash();

        sqlx::query!(
            r#"
                INSERT INTO sessions (user_id, token_hash, created_at, expires_at)
                VALUES (?, ?, ?, ?)
            "#,
            user.id,
            token_hash,
            Utc::now().timestamp(),
            expires_at.timestamp()
        )
        .execute(&mut self.conn)
        .await?;

        Ok(CreatedSessionToken { user, token })
    }

    pub async fn verify_session(&mut self, token: &SessionToken) -> sqlx::Result<Option<User>> {
        let token_hash = token.hash();
        let now = Utc::now();

        let row = sqlx::query!(
            r#"
                SELECT
                    sessions.id as session_id,
                    users.id,
                    users.name,
                    users.key_hash,
                    users.is_admin,
                    users.enabled,
                    users.created_at
                FROM sessions
                INNER JOIN users ON users.id = sessions.user_id
                WHERE sessions.token_hash = ?
                    AND sessions.expires_at > ?
                    AND users.enabled
            "#,
            token_hash,
            now.timestamp()
        )
        .fetch_optional(&mut self.conn)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let expires_at = now + Duration::days(30);
        sqlx::query!(
            r#"
                UPDATE sessions
                SET expires_at = ?
                WHERE id = ?
            "#,
            expires_at.timestamp(),
            row.session_id
        )
        .execute(&mut self.conn)
        .await?;

        let created_at = DateTime::from_timestamp(row.created_at, 0)
            .ok_or_else(|| sqlx::Error::Protocol("invalid created_at timestamp".into()))?;

        Ok(Some(User {
            id: row.id,
            name: row.name,
            key_hash: row.key_hash,
            is_admin: row.is_admin,
            enabled: row.enabled,
            created_at,
        }))
    }

    pub async fn create_spending(
        &mut self,
        spent: Money,
        description: String,
        category: Category,
        user: &User,
        spent_at: DateTime<Utc>,
    ) -> sqlx::Result<Spending> {
        let created_at = Utc::now();

        let spending = sqlx::query!(
            r#"
                INSERT INTO spendings (spent, description, category, user_id, spent_at, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                RETURNING id as "id!"
            "#,
            spent.as_cents(),
            description.as_str(),
            category.as_ref(),
            user.id,
            spent_at.timestamp(),
            created_at.timestamp()
        )
        .fetch_one(&mut self.conn)
        .await?;

        Ok(Spending {
            id: spending.id,
            spent,
            description,
            category,
            user_id: user.id,
            spent_at,
            created_at,
        })
    }

    pub async fn spendings_for_month(
        &mut self,
        year: i32,
        month: Month,
    ) -> sqlx::Result<Vec<Spending>> {
        let month = month.number_from_month();
        let start = Utc
            .with_ymd_and_hms(year, month, 1, 0, 0, 0)
            .single()
            .ok_or_else(|| sqlx::Error::Protocol("invalid month".into()))?;
        let (next_month_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let end = Utc
            .with_ymd_and_hms(next_month_year, next_month, 1, 0, 0, 0)
            .single()
            .ok_or_else(|| sqlx::Error::Protocol("invalid month".into()))?;

        let rows = sqlx::query!(
            r#"
                SELECT
                    id,
                    spent,
                    description,
                    category,
                    user_id,
                    spent_at,
                    created_at
                FROM spendings
                WHERE spent_at >= ? AND spent_at < ?
                ORDER BY spent_at ASC, id ASC
            "#,
            start.timestamp(),
            end.timestamp()
        )
        .fetch_all(&mut self.conn)
        .await?;

        let mut spendings = Vec::with_capacity(rows.len());
        for row in rows {
            let category = row
                .category
                .parse()
                .map_err(|error| sqlx::Error::Decode(Box::new(error)))?;
            let spent_at = DateTime::from_timestamp(row.spent_at, 0)
                .ok_or_else(|| sqlx::Error::Protocol("invalid spent_at timestamp".into()))?;
            let created_at = DateTime::from_timestamp(row.created_at, 0)
                .ok_or_else(|| sqlx::Error::Protocol("invalid created_at timestamp".into()))?;

            spendings.push(Spending {
                id: row.id,
                spent: Money::from_cents(row.spent),
                description: row.description,
                category,
                user_id: row.user_id,
                spent_at,
                created_at,
            });
        }

        Ok(spendings)
    }
}
