use axum::{
    extract::{Request, State},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use cookie::{Cookie, SameSite};

use crate::{auth::SessionToken, AppState};

pub async fn require_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = session_token_from_cookies(&headers)?;
    let user = state
        .db
        .lock()
        .await
        .verify_session(&token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user);

    let mut response = next.run(request).await;
    if response.status().is_success() {
        response.headers_mut().insert(
            SET_COOKIE,
            HeaderValue::from_str(&session_cookie(token.as_base64()))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        );
    }

    Ok(response)
}

fn session_token_from_cookies(headers: &HeaderMap) -> Result<SessionToken, StatusCode> {
    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let token = Cookie::split_parse(cookies)
        .filter_map(Result::ok)
        .find(|cookie| cookie.name() == "session")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    SessionToken::from_base64(token.value()).ok_or(StatusCode::UNAUTHORIZED)
}

pub(super) fn session_cookie(token: impl AsRef<str>) -> String {
    Cookie::build(("session", token.as_ref()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(true)
        .max_age(cookie::time::Duration::days(30))
        .build()
        .to_string()
}
