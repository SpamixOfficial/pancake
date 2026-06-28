use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, routing::post};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use jsonwebtoken::{Header, encode};
use regex::Regex;
use time::Duration;
use tracing::error;

use crate::api::{
    JWT_KEYS,
    models::{
        errors::ApiError,
        etc::UserSession,
        requests::{LoginRequest, SignUpRequest},
        responses::{EmptyResponse, LoginResponse, RESPONSE_OK},
    },
};

use super::ApiState;

pub fn routes(state: ApiState) -> Router<ApiState> {
    Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .with_state(state)
}

async fn sign_up(
    State(mut state): State<ApiState>,
    Json(body): Json<SignUpRequest>,
) -> Result<Json<EmptyResponse>, ApiError> {
    // only way to create a user if signup is disabled is via a POST call to /api/users with admin privileges
    if state.config.signup_disabled {
        return Err(ApiError::Unauthorized);
    }

    // shoutout https://emailregex.com/
    let email_regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();

    if !email_regex.is_match(&body.email) {
        return Err(ApiError::InvalidEmail);
    }

    if state
        .db
        .exists_user(Some(body.username.clone()), Some(body.email.clone()))
        .await
    {
        return Err(ApiError::UserAlreadyExists);
    }

    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|e| {
            error!("Failed to hash password: {e}");
            ApiError::ServerError
        })?
        .to_string();

    state
        .db
        .create_user(body.name, body.username, body.email.clone(), password_hash)
        .await
        .map_err(|e| {
            error!("Failed to create user with email \"{}\": {}", body.email, e);
            ApiError::ServerError
        })?;

    Ok(Json(RESPONSE_OK))
}

async fn login(
    State(state): State<ApiState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
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

    let (refresh_cookie, jwt) = generate_tokens(state.clone(), user.id).await?;

    Ok((jar.add(refresh_cookie), Json(jwt)))
}

async fn refresh_token(
    State(mut state): State<ApiState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    let refresh_token = jar
        .get("refresh_token")
        .ok_or(ApiError::NoToken)?
        .value()
        .to_owned();

    let token = state
        .db
        .get_token(refresh_token)
        .await
        .map_err(|_| ApiError::InvalidToken)?;

    state
        .db
        .purge_token(token.token.clone())
        .await
        .map_err(|e| {
            error!("Failed to purge token ({}): {e}", token.token);
            ApiError::ServerError
        })?;

    let (refresh_cookie, jwt) = generate_tokens(state.clone(), token.user_id).await?;

    Ok((jar.add(refresh_cookie), Json(jwt)))
}

async fn generate_tokens(
    mut state: ApiState,
    user_id: i64,
) -> Result<(Cookie<'static>, LoginResponse), ApiError> {
    let jwt_data = UserSession::new(user_id, None);
    let jwt = encode(&Header::default(), &jwt_data, &JWT_KEYS.encoding).map_err(|e| {
        error!("Failed to create JWT for user {}: {}", user_id, e);
        ApiError::ServerError
    })?;

    let token = hex::encode(rand::random_iter().take(32).collect::<Vec<u8>>());

    let refresh_token = state
        .db
        .create_refresh_token(token, user_id, None)
        .await
        .map_err(|e| {
            error!("Failed to create refresh token for user {}: {}", user_id, e);
            ApiError::ServerError
        })?;

    let cookie = Cookie::build(("refresh_token", refresh_token.token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth/refresh")
        .max_age(Duration::days(30))
        .build();

    Ok((cookie, LoginResponse::new(jwt)))
}
