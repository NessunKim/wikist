use crate::models::{Article, ArticlePermission, Namespace, NamespacePermission, User};
use crate::schema::{article_permissions, namespace_permissions, roles, user_roles};
use anyhow::Result;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug, Eq, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Associations, Identifiable, Insertable, Queryable, Debug)]
#[primary_key(user_id, role_id)]
#[belongs_to(User)]
#[belongs_to(Role)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Generates permission checking methods
macro_rules! permission_checker_for_article {
    ($x:ident) => {
        pub fn $x(&self, conn: &PgConnection, article: &Article) -> Result<bool> {
            let article_permission = ArticlePermission::belonging_to(self)
                .filter(article_permissions::article_id.eq(article.id))
                .filter(article_permissions::$x.is_not_null())
                .first::<ArticlePermission>(conn)
                .optional()?;
            if let Some(article_permission) = article_permission {
                return Ok(article_permission.$x.unwrap());
            }
            let namespace_permission = NamespacePermission::belonging_to(self)
                .filter(namespace_permissions::namespace_id.eq(article.namespace_id))
                .first::<NamespacePermission>(conn)?;
            Ok(namespace_permission.$x)
        }
    };
}

/// Generates permission checking methods
macro_rules! permission_checker_for_namespace {
    ($x:ident) => {
        pub fn $x(&self, conn: &PgConnection, namespace: &Namespace) -> Result<bool> {
            let namespace_permission = NamespacePermission::belonging_to(self)
                .filter(namespace_permissions::namespace_id.eq(namespace.id))
                .first::<NamespacePermission>(conn)?;
            Ok(namespace_permission.$x)
        }
    };
}

impl Role {
    pub fn root() -> Self {
        Self {
            id: 1,
            name: "Root".to_owned(),
        }
    }

    pub fn anonymous() -> Self {
        Self {
            id: 2,
            name: "Anonymous".to_owned(),
        }
    }

    pub fn logged_in() -> Self {
        Self {
            id: 3,
            name: "LoggedIn".to_owned(),
        }
    }

    pub fn create(conn: &PgConnection, name: &str) -> Result<Self> {
        let role = diesel::insert_into(roles::table)
            .values(roles::name.eq(name))
            .get_result(conn)?;
        Ok(role)
    }

    pub fn add_user(&self, conn: &PgConnection, user: &User) -> Result<()> {
        user.add_role(conn, self)
    }

    permission_checker_for_article!(can_read);
    permission_checker_for_article!(can_edit);
    permission_checker_for_article!(can_rename);
    permission_checker_for_article!(can_delete);
    permission_checker_for_namespace!(can_create);
    permission_checker_for_namespace!(can_grant);
}
