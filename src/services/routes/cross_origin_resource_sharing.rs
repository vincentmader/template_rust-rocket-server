use crate::models::api_response::ApiResponse;
use rocket::{
    http::{
        uri::{fmt::Path, Segments},
        Status,
    },
    options,
    serde::json::Value,
};

#[options("/<_path..>", rank = 10)]
pub fn cors_options(_path: Segments<Path>) -> ApiResponse {
    ApiResponse {
        status: Status::Ok,
        json: Value::from(""),
    }
}
