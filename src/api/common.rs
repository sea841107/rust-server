pub use actix_web::{web, HttpResponse, Responder};
pub use serde::{Deserialize, Serialize};

pub use crate::api::constant;
pub use crate::api::error_code;
pub use crate::api::error_msg;

#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct EmptyResponse;

pub fn create_response<T>() -> BaseResponse<T>
where
    T: Serialize,
{
    BaseResponse {
        code: 0,
        msg: String::from(""),
        data: None,
    }
}
