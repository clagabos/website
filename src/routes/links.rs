use actix_web::{get, HttpResponse, Responder};
use askama::Template;
use crate::util::misc::minify_string;

pub(super) fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(route);
}

#[derive(Template)]
#[template(path = "links.html")]
struct LinksTemplate<'a> {
    page_name: &'a str
}

#[get("/links")]
async fn route() -> impl Responder {
    let links = LinksTemplate { page_name: "Links" };

    let mut content = links.render().unwrap();

    content = minify_string(&content);

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .append_header(("Content-Length", content.len().to_string()))
        // cors
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(content)
}