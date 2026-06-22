use crate::api::models::BaseResponseStatus;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::BaseResponse;
pub enum ApiError {
    NoSuchUser,
    InvalidPassword,
    ServerError
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid password"),
            Self::NoSuchUser => (StatusCode::UNAUTHORIZED, "No such user"),
            Self::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        };

        (
            status,
            Json(BaseResponse::<(), &str> {
                status: BaseResponseStatus::Err,
                error,
                data: (),
            }),
        )
            .into_response()
    }
}
