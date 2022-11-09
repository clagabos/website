#[macro_use]
extern crate log;
use dotenv::dotenv;
use actix_web::{App, HttpServer};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
mod routes;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure)
            .wrap(
                    ErrorHandlers::new()
                        .handler(StatusCode::BAD_REQUEST, routes::error::render_400)
                        .handler(StatusCode::NOT_FOUND, routes::error::render_404)
                        .handler(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            routes::error::render_500,
                        ),
                )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}