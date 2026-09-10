use axum::{routing::get, Router};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, SqliteConnection};
use std::str::FromStr;

use crate::db::Db;

mod db;

#[tokio::main]
async fn main() {
    let db = Db::connect().await.unwrap();

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();

    // axum::serve(listener, app).await.unwrap();
}
