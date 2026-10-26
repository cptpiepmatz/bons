mod auth;

use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::{header::SET_COOKIE, HeaderValue, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Duration, Month, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use bons::{
    auth::Key,
    category::Category,
    db::{Spending, User},
    money::Money,
};

use crate::{api::auth::session_cookie, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    let protected_routes = Router::new()
        .route("/me", get(current_user))
        .route("/users", post(create_user))
        .route("/spending", post(create_spending))
        .route("/spending/{id}", delete(delete_spending))
        .route("/spending/{year}/{month}", get(spending_for_month))
        .route_layer(middleware::from_fn_with_state(state, auth::require_session));

    Router::new()
        .route("/currency", get(currency))
        .route("/login", post(login))
        .route("/logout", post(auth::logout))
        .merge(protected_routes)
        .fallback(|| async { StatusCode::NOT_FOUND })
}

async fn currency(State(state): State<AppState>) -> String {
    state.currency.to_string()
}

async fn current_user(Extension(user): Extension<User>) -> Json<UserResponse> {
    Json(UserResponse::from_user(&user))
}

async fn create_user(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if !user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut db = state.db.lock().await;
    let created_user = db
        .create_user(request.name, request.is_admin)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateUserResponse {
        user: UserResponse::from_user(&created_user.user),
        key: created_user.key_as_str(),
    }))
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct CreateUserRequest {
    name: String,
    is_admin: bool,
}

#[derive(Serialize, TS)]
#[ts(export)]
struct CreateUserResponse {
    user: UserResponse,
    key: String,
}

async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let key = Key::from_base64(request.key).ok_or(StatusCode::UNAUTHORIZED)?;
    let mut db = state.db.lock().await;
    let user = db
        .verify_login(&key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let session = db
        .create_session_token(&user, Utc::now() + Duration::days(30))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut response = Json(LoginResponse {
        user: UserResponse::from_user(&user),
    })
    .into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&session_cookie(session.token_as_string()))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct LoginRequest {
    key: String,
}

#[derive(Serialize, TS)]
#[ts(export)]
struct LoginResponse {
    user: UserResponse,
}

async fn create_spending(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<CreateSpendingRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if request.spent_cents < 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let spent_at = DateTime::from_timestamp(request.spent_at, 0).ok_or(StatusCode::BAD_REQUEST)?;

    let mut db = state.db.lock().await;
    let spending = db
        .create_spending(
            Money::from_cents(request.spent_cents),
            request.description,
            request.category,
            &user,
            spent_at,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SpendingResponse::from_spending(&spending)))
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct CreateSpendingRequest {
    #[ts(type = "number")]
    spent_cents: i64,
    description: String,
    category: Category,
    #[ts(type = "number")]
    spent_at: i64,
}

async fn delete_spending(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let mut db = state.db.lock().await;
    let deleted = db
        .delete_spending(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn spending_for_month(
    State(state): State<AppState>,
    Path((year, month)): Path<(i32, String)>,
    Query(query): Query<SpendingForMonthQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let month = Month::from_str(&month).map_err(|_| StatusCode::BAD_REQUEST)?;
    let timezone = query
        .timezone
        .parse::<Tz>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut db = state.db.lock().await;
    let spending = db
        .spending_for_month(year, month, timezone)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        spending
            .iter()
            .map(SpendingResponse::from_spending)
            .collect::<Vec<_>>(),
    ))
}

#[derive(Deserialize)]
struct SpendingForMonthQuery {
    timezone: String,
}

#[derive(Serialize, TS)]
#[ts(export)]
struct UserResponse {
    #[ts(type = "number")]
    id: i64,
    name: String,
    is_admin: bool,
    enabled: bool,
    #[ts(type = "number")]
    created_at: i64,
}

impl UserResponse {
    fn from_user(user: &User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            is_admin: user.is_admin,
            enabled: user.enabled,
            created_at: user.created_at.timestamp(),
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
struct SpendingResponse {
    #[ts(type = "number")]
    id: i64,
    #[ts(type = "number")]
    spent_cents: i64,
    description: String,
    category: Category,
    #[ts(type = "number")]
    user_id: i64,
    user_name: String,
    #[ts(type = "number")]
    spent_at: i64,
    #[ts(type = "number")]
    created_at: i64,
}

impl SpendingResponse {
    fn from_spending(spending: &Spending) -> Self {
        Self {
            id: spending.id,
            spent_cents: spending.spent.as_cents(),
            description: spending.description.clone(),
            category: spending.category,
            user_id: spending.user_id,
            user_name: spending.user_name.clone(),
            spent_at: spending.spent_at.timestamp(),
            created_at: spending.created_at.timestamp(),
        }
    }
}
