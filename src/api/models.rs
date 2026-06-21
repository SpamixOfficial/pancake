//! Response, request and error models

pub mod responses;
pub mod requests;
pub mod errors;
pub mod etc;

use serde::Serialize;

#[derive(Serialize)]
pub enum BaseResponseStatus {
    Ok,
    Err,
}

/// Generic struct for all responses
/// 
/// All responses should follow this format!
#[derive(Serialize)]
pub struct BaseResponse<D: Serialize, E: Serialize> {
    status: BaseResponseStatus,
    error: E,
    data: D,
}