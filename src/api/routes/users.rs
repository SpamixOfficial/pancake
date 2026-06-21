use axum::{Router, response::Response, routing::get};
use hyper::StatusCode;
use tower::layer;

use super::ApiState;

pub fn routes(state: ApiState) -> Router<ApiState> {
    //Router::new().route("/", get(get_users).layer().with_state(state);
}

async fn get_users() {

}