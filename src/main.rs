#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> Template {
    let context = context! { message: "Hello, world!" };
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, index])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
