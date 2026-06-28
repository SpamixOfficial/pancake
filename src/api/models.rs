//! Response, request and error models

pub mod responses;
pub mod requests;
pub mod errors;
pub mod etc;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BaseResponseStatus {
    Ok,
    #[serde(rename = "error")]
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