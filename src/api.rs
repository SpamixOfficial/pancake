//! Main export layer for api, routes are returned through the routes() method!
//!
//! When adding a new api route add it in a **READABLE** way in the routes() method

mod middleware;
mod models;
mod routes;
mod state;

use std::{fs, path::PathBuf, sync::{Arc, LazyLock}};

use axum::{Router, routing::get};
use color_eyre::eyre::{Result, eyre};
use jsonwebtoken::{DecodingKey, EncodingKey};
use tracing::{error, info};

use crate::{config::Config, db::DBClient, exists};

pub use state::ApiState;

static JWT_KEYS: LazyLock<JwtKeys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap();
    JwtKeys {
        encoding: EncodingKey::from_secret(secret.as_bytes()),
        decoding: DecodingKey::from_secret(secret.as_bytes()),
    }
});

struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

pub fn routes(state: ApiState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/users", routes::users::routes(state.clone()))
        .nest("/auth", routes::auth::routes(state.clone()))
        .with_state(state)
}
