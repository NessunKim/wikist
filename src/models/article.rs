use crate::models::{Actor, NewRevision, Revision};
use crate::schema::articles;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, AsChangeset, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub latest_revision_id: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    pub title: &'a str,
    pub latest_revision_id: i32,
}

impl Article {
    pub fn create(conn: &PgConnection, title: &str, wikitext: &str, actor: &Actor) -> Result<Self> {
        if let Some(_) = Self::find_by_full_title(conn, title)? {
            return Err(anyhow!("Article {} already exists", title));
        }
        conn.transaction(|| {
            let new_article = NewArticle {
                title: title,
                latest_revision_id: -1,
            };
            let mut article = diesel::insert_into(articles::table)
                .values(new_article)
                .get_result::<Article>(conn)?;
            let revision = Revision::create(conn, &article, wikitext, actor)?;
            article.set_latest_revision(conn, &revision)?;
            Ok(article)
        })
    }
    pub fn find_by_full_title(
        conn: &PgConnection,
        full_title: &str,
    ) -> Result<Option<Self>, diesel::result::Error> {
        let article = articles::table
            .filter(articles::title.eq(full_title))
            .first::<Article>(conn)
            .optional()?;
        Ok(article)
    }
    pub fn add_null_revision(&mut self, conn: &PgConnection, actor: &Actor) -> Result<Revision> {
        use crate::schema::revisions;
        conn.transaction(|| {
            let now = Utc::now().naive_utc();
            let content = self.get_latest_revision(conn)?.get_content(conn)?;
            let new_revision = NewRevision {
                article_id: self.id,
                actor_id: actor.id,
                content_id: content.id,
                created_at: now,
            };
            let revision = diesel::insert_into(revisions::table)
                .values(new_revision)
                .get_result(conn)?;
            self.set_latest_revision(conn, &revision)?;

            Ok(revision)
        })
    }
    pub fn edit(&mut self, conn: &PgConnection, wikitext: &str, actor: &Actor) -> Result<Revision> {
        conn.transaction(|| {
            let revision = Revision::create(conn, self, wikitext, actor)?;
            self.set_latest_revision(conn, &revision)?;
            Ok(revision)
        })
    }
    /// Set is_active false.
    ///
    /// Creates null revision.
    pub fn delete(&mut self, conn: &PgConnection, actor: &Actor) -> Result<Revision> {
        conn.transaction(|| {
            self.is_active = false;
            self.save_changes::<Self>(conn)?;
            self.add_null_revision(conn, actor)
        })
    }
    pub fn get_latest_revision(&self, conn: &PgConnection) -> Result<Revision> {
        use crate::schema::revisions;
        let latest = revisions::table
            .find(self.latest_revision_id)
            .first::<Revision>(conn)
            .optional()?;
        if let Some(latest) = latest {
            Ok(latest)
        } else {
            Err(anyhow!("Cannot find latest revision"))
        }
    }
    pub fn get_all_revision(&self, conn: &PgConnection) -> Result<Vec<Revision>> {
        use crate::schema::revisions;
        let revisions = Revision::belonging_to(self)
            .order(revisions::id.asc())
            .load::<Revision>(conn)?;
        Ok(revisions)
    }

    /// Creates an `Article` and copies all `Revision`s to the new `Article`.
    ///
    /// Creates null revision.
    pub fn fork(&self, conn: &PgConnection, title: &str, actor: &Actor) -> Result<Self> {
        use crate::schema::revisions;
        conn.transaction(|| {
            let new_article = NewArticle {
                title: title,
                latest_revision_id: -1,
            };
            let mut article = diesel::insert_into(articles::table)
                .values(new_article)
                .get_result::<Article>(conn)?;
            let revisions = self.get_all_revision(conn)?;
            let new_revisions = revisions
                .iter()
                .map(|rev| NewRevision {
                    article_id: article.id,
                    actor_id: rev.actor_id,
                    content_id: rev.content_id,
                    created_at: rev.created_at,
                })
                .collect::<Vec<NewRevision>>();
            let copied_revisions = diesel::insert_into(revisions::table)
                .values(new_revisions)
                .get_results::<Revision>(conn)?;
            let latest_rev = &copied_revisions.last().unwrap();
            article.set_latest_revision(conn, latest_rev)?;
            article.add_null_revision(conn, actor)?;

            Ok(article)
        })
    }

    fn set_latest_revision(&mut self, conn: &PgConnection, revision: &Revision) -> Result<()> {
        let now = Utc::now().naive_utc();
        self.latest_revision_id = revision.id;
        self.updated_at = now;
        self.save_changes::<Self>(conn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_connection;

    #[test]
    fn test_create_article() {
        use ipnetwork::IpNetwork;
        use std::str::FromStr;
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let ip_address = IpNetwork::from_str("127.0.0.1").expect("must succeed");
            let actor = Actor::find_or_create_from_ip(&conn, &ip_address).expect("must succeed");
            Article::create(&conn, "test", "==test==", &actor).expect("must succeed");
            articles::table
                .filter(articles::title.eq("test"))
                .first::<Article>(&conn)
                .expect("must exist");

            Ok(())
        });
    }

    #[test]
    fn test_edit_article() {
        use ipnetwork::IpNetwork;
        use std::str::FromStr;
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let ip_address = IpNetwork::from_str("127.0.0.1").expect("must succeed");
            let actor = Actor::find_or_create_from_ip(&conn, &ip_address).expect("must succeed");
            let mut article =
                Article::create(&conn, "test", "==test==", &actor).expect("must succeed");
            article
                .edit(&conn, "==test-edit==", &actor)
                .expect("must succeed");
            let article = articles::table
                .filter(articles::title.eq("test"))
                .first::<Article>(&conn)
                .expect("must exist");
            let wikitext = article
                .get_latest_revision(&conn)
                .expect("must exist")
                .get_wikitext(&conn)
                .expect("must succeed");
            assert_eq!(wikitext, "==test-edit==");

            Ok(())
        });
    }
}
