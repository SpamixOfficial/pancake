use serde::Serialize;

use super::{BaseResponse, BaseResponseStatus};

pub const RESPONSE_OK: BaseResponse<(), ()> = BaseResponse {
    status: BaseResponseStatus::Ok,
    error: (),
    data: (),
};

pub type EmptyResponse = BaseResponse<(), ()>;

#[derive(Serialize)]
pub struct LoginResponseData {
    token: String,
}

pub type LoginResponse = BaseResponse<LoginResponseData, ()>;

impl LoginResponse {
    pub fn new(token: String) -> Self {
        BaseResponse {
            status: BaseResponseStatus::Ok,
            error: (),
            data: LoginResponseData { token },
        }
    }
}
