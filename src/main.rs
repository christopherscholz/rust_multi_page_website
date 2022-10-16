#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

#[get("/")]
fn home() -> Template {
    Template::render("home", context! {})
}

#[get("/impressum")]
fn impressum() -> Template {
    Template::render("impressum", context! {})
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", context! {})
}

#[cfg(test)] mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![home])
        .mount("/", routes![impressum])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
