use crate::{database::SqliteDb, models::api_response::ApiResponse, services::login_validity};
use rocket::{http::Status, post, serde::json::Json};
use rocket_db_pools::Connection;
use rs_web_api_models::api_message::{ApiError, ApiMessage, ApiOk, LoginError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    user_info: String, // This can be either `user_name` or `mail_address`.
    pass_hash: String, // This hash has been encrypted AT MOST (or ONLY) with SHA256 so far.
}

#[post("/login", data = "<data>")]
pub async fn login(mut conn: Connection<SqliteDb>, data: Json<Request>) -> ApiResponse {
    let user_info = &data.user_info;
    let pass_hash = &data.pass_hash;

    if login_validity::is_valid_login(&mut conn, user_info, pass_hash).await {
        let msg = ApiOk::LoginSucceeded;
        let msg = ApiMessage::Ok(msg);
        ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        }
    } else {
        let msg = LoginError::InvalidLoginCredentials;
        let msg = ApiError::LoginError(msg);
        let msg = ApiMessage::Err(msg);
        ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        }
    }
}
