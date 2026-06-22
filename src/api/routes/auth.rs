use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, Router, extract::State, routing::post};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use jsonwebtoken::{Header, encode};
use time::Duration;
use tracing::error;

use crate::api::{
    JWT_KEYS,
    models::{
        errors::ApiError, etc::UserSession, requests::LoginRequest, responses::LoginResponse,
    },
};

use super::ApiState;

pub fn routes(state: ApiState) -> Router<ApiState> {
    Router::new().route("/login", post(login)).with_state(state);
}

async fn login(
    State(mut state): State<ApiState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let user = state
        .db
        .get_user_by_email(body.email)
        .await
        .map_err(|_| ApiError::NoSuchUser)?;

    let hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        error!("Failed to hash password: {}", e);
        ApiError::ServerError
    })?;
    Argon2::default()
        .verify_password(body.password.as_bytes(), &hash)
        .map_err(|_| ApiError::InvalidPassword)?;

    let jwt_data = UserSession::new(user.id, None);
    let jwt = encode(&Header::default(), &jwt_data, &JWT_KEYS.encoding).map_err(|e| {
        error!("Failed to create JWT for user {}: {}", user.id, e);
        ApiError::ServerError
    })?;

    let refresh_token = state
        .db
        .new_refresh_token(user.id, None)
        .await
        .map_err(|e| {
            error!("Failed to create refresh token for user {}: {}", user.id, e);
            ApiError::ServerError
        })?;

    let cookie = Cookie::build(("refresh_token", refresh_token.token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth/refresh")
        .max_age(Duration::days(30));
    
    jar.add(cookie);

    Ok(Json(LoginResponse::new(jwt)))
}
