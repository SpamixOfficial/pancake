//! Main export layer for api, routes are returned through the routes() method!
//!
//! When adding a new api route add it in a **READABLE** way in the routes() method

mod middleware;
mod models;
mod routes;
mod state;

use std::{fs, path::PathBuf, sync::Arc};

use axum::{Router, routing::get};
use color_eyre::eyre::{Result, eyre};
use tracing::{error, info};

use crate::{config::Config, db::DBClient, exists};

pub use state::ApiState;

pub fn routes(state: ApiState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/users", routes::users::routes(state.clone()))
        .nest("/auth", routes::auth::routes(state.clone()))
        .with_state(state)
}
