use actix_web::{get, HttpResponse, Responder};
use askama::Template;

pub(super) fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(route);
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    page_name: &'a str
}

#[get("/")]
async fn route() -> impl Responder {
    let home = HomeTemplate { page_name: "Home" };

    let content = home.render().unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .append_header(("Content-Length", content.len().to_string()))
        // cors
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(content)
}