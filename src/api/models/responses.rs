use super::{BaseResponse, BaseResponseStatus};

pub const RESPONSE_OK: BaseResponse<(), ()> = BaseResponse {
    status: BaseResponseStatus::Ok,
    error: (),
    data: (),
};


pub type EmptyResponse = BaseResponse<(), ()>;