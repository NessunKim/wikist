#[macro_use]
extern crate diesel;
extern crate diesel_full_text_search;
#[macro_use]
extern crate validator_derive;
extern crate validator;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_validator::JsonConfig;

pub mod auth;
pub mod db;
pub mod extractors;
pub mod models;
pub mod parser;
pub mod renderer;
pub mod routes;
pub mod schema;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, Result};

fn render_500(res: dev::ServiceResponse) -> Result<ErrorHandlerResponse<dev::Body>> {
    if let Some(e) = res.response().error() {
        eprintln!("{}", e);
    }
    Ok(ErrorHandlerResponse::Response(res.map_body(
        |_head, _body| {
            dev::ResponseBody::Body(dev::Body::Message(Box::new("Internal Server Error")))
        },
    )))
}

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let pool = db::create_connection_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().limit(1024 * 1024 * 50))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .wrap(Logger::default())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    //.allowed_origin("http://localhost:8000/")
                    .max_age(3600)
                    .finish(),
            )
            .data(pool.clone())
            .service(routes::index)
            .service(routes::articles::get_article)
            .service(routes::articles::get_revisions)
            .service(routes::articles::edit_article)
            .service(routes::articles::create_article)
            .service(routes::articles::delete_article)
            .service(routes::auth::auth_facebook)
            .service(routes::auth::refresh)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
