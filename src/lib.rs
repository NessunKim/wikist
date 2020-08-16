#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
mod db;
mod models;
mod parser;
mod renderer;
mod routes;
mod schema;

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let pool = db::create_connection_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    //.allowed_origin("http://localhost:8000/")
                    .max_age(3600)
                    .finish(),
            )
            .service(routes::index)
            .service(routes::articles::get_by_full_title)
            .service(routes::articles::create_article)
            .service(routes::auth::auth_facebook)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
