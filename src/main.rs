#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template};

mod database;
mod models;
mod routes;

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!("/static/favicon.ico"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::db::stage())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![favicon])
        .attach(routes::home::stage())
        .attach(routes::accounts::stage())
        .attach(Template::custom(|_| {}))
}
