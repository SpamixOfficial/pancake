use axum::{Json, Router, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;

use crate::api::models::{
    requests::LoginRequest,
    responses::{EmptyResponse, RESPONSE_OK},
};

use super::ApiState;

pub fn routes(state: ApiState) -> Router<ApiState> {
    Router::new()
}

async fn login(
    State(state): State<ApiState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<Json<EmptyResponse>, StatusCode> {
    
    let user = state.db.authenticate_user_by_email(body.email, body.password).await?;
    

    Ok(Json(RESPONSE_OK))
}
