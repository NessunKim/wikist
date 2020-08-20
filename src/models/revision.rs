use crate::models::{actor::Actor, article::Article};
use crate::schema::{contents, revisions};
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Article)]
pub struct Revision {
    pub id: i32,
    pub article_id: i32,
    pub actor_id: i32,
    pub content_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "revisions"]
struct NewRevision {
    pub article_id: i32,
    pub actor_id: i32,
    pub content_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Content {
    pub id: i32,
    pub wikitext: String,
}

#[derive(Insertable)]
#[table_name = "contents"]
struct NewContent<'a> {
    pub wikitext: &'a str,
}

impl Revision {
    pub fn create(
        conn: &PgConnection,
        article: &Article,
        actor: &Actor,
        wikitext: &str,
    ) -> Result<Revision> {
        let now = Utc::now().naive_utc();
        let content = diesel::insert_into(contents::table)
            .values(NewContent { wikitext })
            .get_result::<Content>(conn)?;
        let new_revision = NewRevision {
            article_id: article.id,
            actor_id: actor.id,
            content_id: content.id,
            created_at: now,
        };
        let revision = diesel::insert_into(revisions::table)
            .values(new_revision)
            .get_result(conn)?;
        Ok(revision)
    }
    pub fn get_wikitext(self, conn: &PgConnection) -> Result<String> {
        Ok(self.get_content(conn)?.wikitext)
    }
    fn get_content(self, conn: &PgConnection) -> Result<Content> {
        let content = contents::table
            .find(self.content_id)
            .first::<Content>(conn)
            .optional()?;
        if let Some(content) = content {
            Ok(content)
        } else {
            Err(anyhow!("Cannot find content of the revision"))
        }
    }
}
