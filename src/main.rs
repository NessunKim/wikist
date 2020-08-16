extern crate wikist;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    wikist::run().await
}
