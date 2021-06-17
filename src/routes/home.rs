use rocket::fairing::AdHoc;
use rocket::serde::Serialize;
use rocket_dyn_templates::{Template};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext<'r> {
    title: &'r str,
    name: Option<&'r str>,
    items: Vec<&'r str>,
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render("home/index", &TemplateContext {
        title: "Hello",
        name: Some("Test"),
        items: vec!["One", "Two", "Three"],
        parent: "layout",
    })
}

#[get("/about")]
fn about() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("title", "About");
    map.insert("parent", "layout");
    Template::render("home/about", &map)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.mount("/", routes![index, about])
    })
}
