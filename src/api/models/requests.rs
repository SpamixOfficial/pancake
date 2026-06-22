use sea_orm::StringColumn;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct TokenRefreshRequest {
    pub refresh_token: String
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String
}