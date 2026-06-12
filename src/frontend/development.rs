// A lot of this code was borrowed from the axum reverse proxy example
// https://github.com/tokio-rs/axum/tree/main/examples/reverse-proxy

use axum::{
    body::Body,
    extract::{Request, State},
    http::uri::Uri,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use hyper_util::{client::legacy::connect::HttpConnector};
use dotenvy_macro::dotenv;
pub type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn get_frontend_service(
    State(client): State<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    // compiletime is fine here, just development mode
    let frontend_url = dotenv!("FRONTEND_URL");
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|x| x.as_str())
        .unwrap_or(path);

    let uri = format!("{frontend_url}{path_query}");
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
