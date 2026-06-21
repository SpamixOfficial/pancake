//! Request middleware like authentication checks

use std::sync::LazyLock;

use super::state::ApiState;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Result;
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::db::entity;

static JWT_KEYS: LazyLock<JwtKeys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap();
    JwtKeys {
        encoding: EncodingKey::from_secret(secret.as_bytes()),
        decoding: DecodingKey::from_secret(secret.as_bytes()),
    }
});

async fn authenticated(
    State(state): State<ApiState>,
    headers: HeaderMap,
    cookies: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
}

async fn admin(
    State(state): State<ApiState>,
    headers: HeaderMap,
    cookies: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
}

async fn get_user(
    state: ApiState,
    headers: HeaderMap,
    cookies: CookieJar,
) -> Result<entity::prelude::User> {
    
}

struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}
