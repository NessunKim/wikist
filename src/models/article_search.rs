use crate::models::Article;
use crate::schema::article_searches;
use anyhow::Result;
use diesel::prelude::*;
use diesel_full_text_search::{to_tsvector, TsVector};
use voca_rs::strip::strip_tags;

#[derive(Associations, Identifiable, Queryable)]
#[table_name = "article_searches"]
#[primary_key(article_id)]
#[belongs_to(Article)]
pub struct ArticleSearch {
    pub article_id: i32,
    pub vector: TsVector,
}

impl ArticleSearch {
    pub fn create(conn: &PgConnection, article: &Article) -> Result<()> {
        let search_vector = strip_tags(&article.get_html(conn)?);
        diesel::insert_into(article_searches::table)
            .values((
                article_searches::article_id.eq(article.id),
                article_searches::vector.eq(to_tsvector(search_vector)),
            ))
            .execute(conn)?;
        Ok(())
    }
}
