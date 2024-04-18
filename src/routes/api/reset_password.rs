use crate::{database::SqliteDb, models::api_response::ApiResponse, utils::login_validity};
use rocket::{http::Status, post, serde::json::Json};
use rocket_db_pools::Connection;
use rs_web_api_models::api_message::{ApiError, ApiMessage, ApiOk, PasswordResetError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    user_info: String, // This can be either `user_name` or `mail_address`.
}

#[post("/reset_password", data = "<data>")]
pub async fn reset_password(mut conn: Connection<SqliteDb>, data: Json<Request>) -> ApiResponse {
    let user_info = &data.user_info;

    let is_existing_user_name = login_validity::is_existing_user_name(&mut conn, user_info).await;
    let is_existing_mail_addr = login_validity::is_existing_mail_addr(&mut conn, user_info).await;

    if is_existing_user_name || is_existing_mail_addr {
        // TODO: Send password reset email.
        let msg = ApiOk::PasswordResetMailWasSent;
        let msg = ApiMessage::Ok(msg);
        ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        }
    } else {
        let msg = PasswordResetError::AccountDoesNotExist;
        let msg = ApiError::PasswordResetError(msg);
        let msg = ApiMessage::Err(msg);
        ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        }
    }
}
