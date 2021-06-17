#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::{handlebars, Template};

use self::handlebars::{Handlebars, JsonRender};

mod routes;

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!("/static/favicon.ico"))
}

fn wow_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn customize(hbs: &mut Handlebars) {
    hbs.register_helper("wow", Box::new(wow_helper));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from("static"))
        .mount( "/", routes![favicon])
        .attach(routes::home::stage())
        .attach(Template::custom(|engines| {
            customize(&mut engines.handlebars);
        }))
}
