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
use time::Duration;
use tracing::error;

use crate::api::{
    JWT_KEYS,
    models::{
        errors::ApiError,
        etc::UserSession,
        requests::{LoginRequest, SignUpRequest, TokenRefreshRequest},
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

    if state.db.get_user_by_email(body.email.clone()).await.is_ok() {
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
        .create_user(body.name, body.email.clone(), password_hash)
        .await
        .map_err(|e| {
            error!("Failed to create user with email \"{}\": {}", body.email, e);
            ApiError::ServerError
        })?;

    Ok(Json(RESPONSE_OK))
}

async fn login(
    State(mut state): State<ApiState>,
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
    Json(body): Json<TokenRefreshRequest>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    let token = state
        .db
        .get_token(body.refresh_token)
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
        .max_age(Duration::days(30)).build();

    Ok((cookie, LoginResponse::new(jwt)))
}
