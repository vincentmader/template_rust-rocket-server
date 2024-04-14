use crate::{database::SqliteDb, models::api_response::ApiResponse, utils::login_validity};
use rocket::{
    http::Status,
    post,
    serde::json::{json, Json},
};
use rocket_db_pools::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    user_info: String, // This can be either `user_name` or `mail_address`.
    pass_hash: String,
}

#[post("/login", data = "<data>")]
pub async fn login(mut conn: Connection<SqliteDb>, data: Json<Request>) -> ApiResponse {
    let user_info = &data.user_info;
    let pass_hash = &data.pass_hash;

    if login_validity::is_valid_login(&mut conn, user_info, pass_hash).await {
        ApiResponse {
            json: json!("Login succeeded."),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            json: json!("Invalid login credentials."),
            status: Status::Unauthorized,
        }
    }
}
