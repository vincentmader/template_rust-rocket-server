#[macro_use]
extern crate rocket;
extern crate bcrypt;
use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use rocket_server::{database::SqliteDb, models::cross_origin_resource_sharing, services::routes};

#[launch]
fn rocket() -> _ {
    let screen_routes = routes![routes::screens::index::index];
    let api_routes = routes![
        routes::api::register::register,
        routes::api::login::login,
        routes::api::reset_password::reset_password,
        routes::api::upload_file::upload_file,
        routes::cross_origin_resource_sharing::cors_options,
    ];

    rocket::build()
        .mount("/api/", api_routes)
        .mount("/", screen_routes)
        .mount("/", FileServer::from(relative!("static")))
        .attach(cross_origin_resource_sharing::cors())
        .attach(SqliteDb::init())
        .attach(Template::fairing())
}
