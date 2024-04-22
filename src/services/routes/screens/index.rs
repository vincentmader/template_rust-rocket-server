use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    let context = context! { message: "Hello, world!" };
    Template::render("index", context)
}
