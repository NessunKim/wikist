#[macro_use]
extern crate diesel;

use actix_web::{middleware::Logger, App, HttpServer};
mod db;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let pool = db::create_connection_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(routes::index)
            .service(routes::articles::get_by_full_title)
            .service(routes::articles::post)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
