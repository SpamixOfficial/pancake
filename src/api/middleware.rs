//! Request middleware like authentication checks

use super::state::ApiState;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Result;

use crate::db::entity;


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
