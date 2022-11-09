use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::ServiceResponse;
use actix_web::http::{header, header::HeaderValue, StatusCode};
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::Result;
use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    page_name: &'a str,
    status: StatusCode,
}

// TODO this is kind of all over the place, could do with revisiting later. works for now, though.

pub fn error_document<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let body = BoxBody::new(
        ErrorTemplate {
            page_name: "Error",
            status: res.status()
        }
        .to_string(),
    );

    let mut res: ServiceResponse<EitherBody<B>> =
        res.map_body(|_, _| EitherBody::<B, BoxBody>::right(body));

    let headers = res.response_mut().headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));

    // stop proxies from caching error responses
    headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));

    // cors
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));

    Ok(ErrorHandlerResponse::Response(res))
}

pub fn render_400<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    error_document::<B>(res)
}

pub fn render_404<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    error_document::<B>(res)
}

pub fn render_500<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    error_document::<B>(res)
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use actix_web::web;
    use actix_web::App;
    use actix_web::middleware::ErrorHandlers;
    use uuid::Uuid;
    use super::*;

    #[actix_rt::test]
    async fn test_404() {
        // Start a test server
        let mut app = test::init_service(
            App::new()
                .wrap(
                    ErrorHandlers::new()
                        .handler(StatusCode::NOT_FOUND, render_404)
                )
                .service(web::resource("/").route(web::get().to(|| async { "Hello world!" })))
        ).await;

        let random_path = format!("/test404/{}", Uuid::new_v4().to_string()); // random path to trigger 404
        let req = test::TestRequest::get().uri(&random_path).to_request(); 
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}