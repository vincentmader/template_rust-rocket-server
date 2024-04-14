use crate::utils::login_validity;
use crate::{database::SqliteDb, models::api_response::ApiResponse};
use rocket::{
    http::Status,
    post,
    serde::json::{json, Json},
};
use rocket_db_pools::{sqlx::Executor, Connection};
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

    if login_validity::is_existing_user_name(&mut conn, user_name).await {
        return ApiResponse {
            json: json!("Account with this username already exists."),
            status: Status::BadRequest,
        };
    }
    if login_validity::is_existing_mail_addr(&mut conn, mail_addr).await {
        return ApiResponse {
            json: json!("Account with this mail address already exists."),
            status: Status::BadRequest,
        };
    }
    if !login_validity::is_valid_mail_addr(mail_addr) {
        return ApiResponse {
            json: json!("Invalid mail address format."),
            status: Status::BadRequest,
        };
    }

    if conn.execute(sql).await.is_ok() {
        ApiResponse {
            json: json!("Registration succeeded."),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            json: json!("Could not save to database."),
            status: Status::InternalServerError,
        }
    }
}
