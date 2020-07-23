use super::schema::articles;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub wikitext: String,
}

#[derive(Insertable)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub wikitext: &'a str,
}
