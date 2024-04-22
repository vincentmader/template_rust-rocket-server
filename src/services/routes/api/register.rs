use crate::{
    database::SqliteDb,
    models::api_response::ApiResponse,
    services::{hashing::generate_hashed_password_and_salt, login_validity},
};
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
    pass_hash: String, // This hash has been encrypted AT MOST (or ONLY) with SHA256 so far.
}

#[post("/register", data = "<data>")]
pub async fn register(mut conn: Connection<SqliteDb>, data: Json<Request>) -> ApiResponse {
    let user_name = &data.user_name;
    let mail_addr = &data.mail_addr;
    let pass_hash = &data.pass_hash; // Now we will apply additional hashing with bcrypt.

    let hash_parts = generate_hashed_password_and_salt(pass_hash);
    let version = bcrypt::Version::TwoB; // NOTE: This might have to be updated at some point.
    let pass_hash = hash_parts.format_for_version(version);

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
