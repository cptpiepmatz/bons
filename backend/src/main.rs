use std::sync::Arc;

use axum::Router;
use tokio::sync::Mutex;

use crate::db::Db;

mod api;
mod auth;
mod category;
mod db;
mod money;

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Db>>,
}

#[tokio::main]
async fn main() {
    let db = Db::connect().await.unwrap();
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let app = Router::new()
        .nest("/api", api::router(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
