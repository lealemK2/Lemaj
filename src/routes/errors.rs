use rocket::fairing::AdHoc;
use rocket::Request;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[catch(404)]
fn not_found(_: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("title", "Not Found");
    map.insert("parent", "layout");
    Template::render("errors/404", &map)
}

#[catch(500)]
fn internal_server_error(_: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("title", "Internat Server Error");
    map.insert("parent", "layout");
    Template::render("errors/500", &map)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Home Stage", |rocket| async {
        rocket.register("/", catchers![not_found, internal_server_error])
    })
}
