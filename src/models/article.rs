use crate::schema::articles;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub wikitext: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    pub title: &'a str,
    pub wikitext: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

pub fn create_article(conn: &PgConnection, title: &str, wikitext: &str) -> Result<Article> {
    let now = Utc::now().naive_utc();
    match get_article_by_full_title(conn, title)? {
        Some(_) => Err(anyhow!("Article {} already exists", title)),
        None => {
            let new_article = NewArticle {
                title: title,
                wikitext: wikitext,
                created_at: now,
                updated_at: now,
            };
            let article = diesel::insert_into(articles::table)
                .values(new_article)
                .get_result(conn)?;
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
            create_article(&conn, "test", "==test==").expect("must succeed");
            articles::table
                .filter(articles::title.eq("test"))
                .first::<Article>(&conn)
                .expect("must exist");

            Ok(())
        });
    }
}
