use crate::schema::articles;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub wikitext: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub wikitext: &'a str,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

pub fn get_article_by_full_title(
    conn: &PgConnection,
    full_title: &str,
) -> Result<Option<Article>, diesel::result::Error> {
    let article = articles::table
        .filter(articles::title.eq(full_title))
        .first::<Article>(conn)
        .optional()?;

    Ok(article)
}

pub fn create_article(conn: &PgConnection, new_article: &NewArticle) -> Result<Article> {
    match get_article_by_full_title(conn, new_article.title)? {
        Some(_) => Err(anyhow!("Article {}  already exists", new_article.title)),
        None => {
            diesel::insert_into(articles::table)
                .values(new_article)
                .execute(conn)?;
            let article = get_article_by_full_title(conn, new_article.title)?
                .expect("Failed to create article");
            Ok(article)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_connection;

    #[actix_rt::test]
    async fn test_create_article() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let new_article = NewArticle {
                title: "test",
                wikitext: "==test==",
                created_at: SystemTime::now(),
                updated_at: SystemTime::now(),
            };
            create_article(&conn, &new_article).expect("must succeed");
            articles::table
                .filter(articles::title.eq("test"))
                .first::<Article>(&conn)
                .expect("must exist");

            Ok(())
        });
    }
}
