use crate::models::{Actor, Article, Namespace};
use crate::schema::redirections;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, AsChangeset, Debug, Associations)]
#[belongs_to(Article, foreign_key = "target_id")]
pub struct Redirection {
    pub id: i32,
    pub namespace_id: i32,
    pub title: String,
    pub target_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "redirections"]
struct NewRedirection<'a> {
    pub namespace_id: i32,
    pub title: &'a str,
    pub target_id: i32,
}

impl Redirection {
    pub fn find(conn: &PgConnection, namespace: &Namespace, title: &str) -> Result<Option<Self>> {
        let redirection = redirections::table
            .filter(redirections::namespace_id.eq(namespace.id))
            .filter(redirections::title.eq(title))
            .first::<Self>(conn)
            .optional()?;
        Ok(redirection)
    }
    pub fn create(
        conn: &PgConnection,
        target: &mut Article,
        namespace: &Namespace,
        title: &str,
        comment: &str,
        actor: &Actor,
    ) -> Result<Self> {
        if let Some(_) = Self::find(conn, namespace, title)? {
            return Err(anyhow!(
                "Redirection {} already exists",
                Namespace::join(namespace, title)
            ));
        }
        if let Some(_) = Article::find(conn, namespace, title)? {
            return Err(anyhow!(
                "Article {} already exists",
                Namespace::join(namespace, title)
            ));
        }
        conn.transaction(|| {
            let new_redirection = NewRedirection {
                namespace_id: namespace.id,
                title,
                target_id: target.id,
            };
            let redirection = diesel::insert_into(redirections::table)
                .values(new_redirection)
                .get_result::<Self>(conn)?;
            target.add_null_revision(
                conn,
                &format!(
                    "(Add redirection: <- {}) {}",
                    Namespace::join(namespace, title),
                    comment
                ),
                actor,
            )?;
            Ok(redirection)
        })
    }
}
