use axum::{
    extract::{Request, State},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use cookie::{Cookie, SameSite};
use std::env;

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
        .secure(env::var_os("INSECURE_COOKIES").is_none())
        .max_age(cookie::time::Duration::days(30))
        .build()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn session_cookie_is_secure_by_default() {
        let _guard = ENV_LOCK.lock().unwrap();
        let previous = env::var_os("INSECURE_COOKIES");
        env::remove_var("INSECURE_COOKIES");

        let cookie = session_cookie("token");

        assert!(cookie.contains("session=token"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("Max-Age=2592000"));

        if let Some(previous) = previous {
            env::set_var("INSECURE_COOKIES", previous);
        }
    }

    #[test]
    fn session_cookie_can_be_insecure_for_local_development() {
        let _guard = ENV_LOCK.lock().unwrap();
        let previous = env::var_os("INSECURE_COOKIES");
        env::set_var("INSECURE_COOKIES", "1");

        let cookie = session_cookie("token");

        assert!(cookie.contains("session=token"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(!cookie.contains("Secure"));
        assert!(cookie.contains("Max-Age=2592000"));

        if let Some(previous) = previous {
            env::set_var("INSECURE_COOKIES", previous);
        } else {
            env::remove_var("INSECURE_COOKIES");
        }
    }
}
