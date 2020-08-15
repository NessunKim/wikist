use super::schema::articles;
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
