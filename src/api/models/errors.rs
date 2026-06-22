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
    InvalidToken,
    UserAlreadyExists,
    Unauthorized,
    ServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid password"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid refresh token"),
            Self::NoSuchUser => (StatusCode::UNAUTHORIZED, "No such user"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "A user with that email already exists"),
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
