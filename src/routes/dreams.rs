use actix_web::{get, HttpResponse, Responder};
use askama::Template;
use crate::util::misc::minify_string;

pub(super) fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(route);
}

#[derive(Template)]
#[template(path = "dreams.html")]
struct DreamsTemplate<'a> {
    page_name: &'a str
}

#[get("/dreams")]
async fn route() -> impl Responder {
    let dreams = DreamsTemplate { page_name: "Dreams" };

    let mut content = dreams.render().unwrap();

    content = minify_string(&content);

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .append_header(("Content-Length", content.len().to_string()))
        // cors
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(content)
}