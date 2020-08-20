use crate::models::{Actor, Revision};
use crate::schema::articles;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub latest_revision_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    pub title: &'a str,
    pub latest_revision_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Article {
    pub fn create(conn: &PgConnection, title: &str, wikitext: &str, actor: &Actor) -> Result<Self> {
        if let Some(_) = Self::find_by_full_title(conn, title)? {
            return Err(anyhow!("Article {} already exists", title));
        }
        let now = Utc::now().naive_utc();
        conn.transaction(|| {
            let new_article = NewArticle {
                title: title,
                latest_revision_id: -1,
                created_at: now,
                updated_at: now,
            };
            let article = diesel::insert_into(articles::table)
                .values(new_article)
                .get_result::<Article>(conn)?;
            let revision = Revision::create(conn, &article, actor, wikitext)?;
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
    fn set_latest_revision(&self, conn: &PgConnection, revision: &Revision) -> Result<()> {
        diesel::update(articles::table)
            .set(articles::latest_revision_id.eq(revision.id))
            .execute(conn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_connection;

    #[actix_rt::test]
    async fn test_create_article() {
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
}
