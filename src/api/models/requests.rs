use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String
}