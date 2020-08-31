use crate::models::{Actor, Article, Namespace, Revision};
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
    ) -> Result<(Self, Revision)> {
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
            let revision = target.add_null_revision(
                conn,
                &format!(
                    "(Add redirection: <- {}) {}",
                    Namespace::join(namespace, title),
                    comment
                ),
                actor,
            )?;
            Ok((redirection, revision))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_connection;

    #[test]
    fn test_create_redirection() {
        use ipnetwork::IpNetwork;
        use std::str::FromStr;
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let ip_address = IpNetwork::from_str("127.0.0.1").expect("must succeed");
            let actor = Actor::find_or_create_from_ip(&conn, &ip_address).expect("must succeed");
            let mut article = Article::create(
                &conn,
                &Namespace::default(),
                "test",
                "==test==",
                "Comment!",
                &actor,
            )
            .expect("must succeed");

            let (_redirection, rev) = Redirection::create(
                &conn,
                &mut article,
                &Namespace::default(),
                "redirection_test",
                "redirection comment",
                &actor,
            )
            .expect("must succeed");
            assert_eq!(
                rev.comment,
                "(Add redirection: <- redirection_test) redirection comment"
            );
            Ok(())
        });
    }
}
