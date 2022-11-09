use actix_web::{get, HttpResponse, Responder};
use askama::Template;
use crate::util::misc::minify_string;

pub(super) fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(route);
}

#[derive(Template)]
#[template(path = "about-me.html")]
struct AboutTemplate<'a> {
    page_name: &'a str
}

#[get("/about")]
async fn route() -> impl Responder {
    let about = AboutTemplate { page_name: "About" };

    let mut content = about.render().unwrap();

    content = minify_string(&content);

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .append_header(("Content-Length", content.len().to_string()))
        // cors
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(content)
}