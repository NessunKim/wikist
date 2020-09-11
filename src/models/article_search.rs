use crate::models::Article;
use crate::schema::{article_searches, articles};
use anyhow::Result;
use diesel::prelude::*;
use diesel_full_text_search::{plainto_tsquery, to_tsvector, TsVector, TsVectorExtensions};
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

    pub fn update(conn: &PgConnection, article: &Article) -> Result<()> {
        let search_vector = strip_tags(&article.get_html(conn)?);
        diesel::update(article_searches::table.find(article.id))
            .set(article_searches::vector.eq(to_tsvector(search_vector)))
            .execute(conn)?;
        Ok(())
    }

    pub fn delete(conn: &PgConnection, article: &Article) -> Result<()> {
        diesel::delete(article_searches::table.find(article.id)).execute(conn)?;
        Ok(())
    }

    pub fn search(conn: &PgConnection, query: &str) -> Result<Vec<Article>> {
        let res = article_searches::table
            .inner_join(articles::table)
            .select(articles::all_columns)
            .filter(article_searches::vector.matches(plainto_tsquery(query)))
            .load::<Article>(conn)?;
        Ok(res)
    }
}
