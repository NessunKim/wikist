pub mod models;
pub mod schema;

use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use models::*;
use schema::articles::dsl::*;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_article_by_full_title(
    conn: &SqliteConnection,
    full_title: &str,
) -> Result<Option<Article>, diesel::result::Error> {
    let article = articles
        .filter(title.eq(full_title))
        .first::<Article>(conn)
        .optional()?;

    Ok(article)
}

pub fn create_article(conn: &SqliteConnection, new_article: &NewArticle) -> Result<Article> {
    match get_article_by_full_title(conn, new_article.title)? {
        Some(_) => Err(anyhow!("Article {}  already exists", new_article.title)),
        None => {
            diesel::insert_into(articles)
                .values(new_article)
                .execute(conn)?;
            let article = get_article_by_full_title(conn, new_article.title)?
                .expect("Failed to create article");
            Ok(article)
        }
    }
}
