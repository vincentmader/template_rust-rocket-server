use crate::utils::login_validity;
use crate::{database::SqliteDb, models::api_response::ApiResponse};
use rocket::{http::Status, post, serde::json::Json};
use rocket_db_pools::{sqlx::Executor, Connection};
use rs_web_api_models::api_message::{
    ApiError, ApiMessage, ApiOk, DatabaseError, RegistrationError,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    user_name: String,
    mail_addr: String,
    pass_hash: String,
}

#[post("/register", data = "<data>")]
pub async fn register(mut conn: Connection<SqliteDb>, data: Json<Request>) -> ApiResponse {
    let user_name = &data.user_name;
    let mail_addr = &data.mail_addr;
    let pass_hash = &data.pass_hash;

    let sql = sqlx::query("INSERT INTO users (user_name,mail_addr,pass_hash) VALUES (?,?,?);")
        .bind(user_name)
        .bind(mail_addr)
        .bind(pass_hash);

    if user_name.is_empty() {
        let msg = RegistrationError::EmptyUserName;
        let msg = ApiError::RegistrationError(msg);
        let msg = ApiMessage::Err(msg);
        return ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        };
    } else if !rs_web_api_models::validation::is_valid_user_name(user_name) {
        let msg = RegistrationError::InvalidUserNameFormat;
        let msg = ApiError::RegistrationError(msg);
        let msg = ApiMessage::Err(msg);
        return ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        };
    } else if login_validity::is_existing_user_name(&mut conn, user_name).await {
        let msg = RegistrationError::UserNameExistsAlready;
        let msg = ApiError::RegistrationError(msg);
        let msg = ApiMessage::Err(msg);
        return ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        };
    } else if mail_addr.is_empty() {
        let msg = RegistrationError::EmptyMailAddress;
        let msg = ApiError::RegistrationError(msg);
        let msg = ApiMessage::Err(msg);
        return ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        };
    } else if !rs_web_api_models::validation::is_valid_mail_addr(mail_addr) {
        let msg = RegistrationError::InvalidMailAddressFormat;
        let msg = ApiError::RegistrationError(msg);
        let msg = ApiMessage::Err(msg);
        return ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        };
    } else if login_validity::is_existing_mail_addr(&mut conn, mail_addr).await {
        let msg = RegistrationError::MailAddressExistsAlready;
        let msg = ApiError::RegistrationError(msg);
        let msg = ApiMessage::Err(msg);
        return ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        };
    }

    if conn.execute(sql).await.is_ok() {
        let msg = ApiOk::RegistrationSucceeded;
        let msg = ApiMessage::Ok(msg);
        ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::Ok,
        }
    } else {
        let msg = DatabaseError::CouldNotSaveToDatabase;
        let msg = ApiError::DatabaseError(msg);
        let msg = ApiMessage::Err(msg);
        ApiResponse {
            json: serde_json::to_value(msg).unwrap(),
            status: Status::InternalServerError,
        }
    }
}
