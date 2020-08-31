use crate::models::{Article, ArticlePermission, Namespace, NamespacePermission, User};
use crate::schema::{article_permissions, namespace_permissions, roles, user_roles};
use anyhow::Result;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
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

impl Role {
    pub fn create(conn: &PgConnection, name: &str) -> Result<Self> {
        let role = diesel::insert_into(roles::table)
            .values(roles::name.eq(name))
            .get_result(conn)?;
        Ok(role)
    }

    pub fn add_user(&self, conn: &PgConnection, user: &User) -> Result<()> {
        user.add_role(conn, self)
    }

    pub fn can_read(&self, conn: &PgConnection, article: &Article) -> Result<bool> {
        let article_permission = ArticlePermission::belonging_to(self)
            .filter(article_permissions::article_id.eq(article.id))
            .filter(article_permissions::can_read.is_not_null())
            .first::<ArticlePermission>(conn)
            .optional()?;
        if let Some(article_permission) = article_permission {
            return Ok(article_permission.can_read.unwrap());
        }

        let namespace_permission = NamespacePermission::belonging_to(self)
            .filter(namespace_permissions::namespace_id.eq(article.namespace_id))
            .first::<NamespacePermission>(conn)?;
        Ok(namespace_permission.can_read)
    }

    pub fn can_edit(&self, conn: &PgConnection, article: &Article) -> Result<bool> {
        let article_permission = ArticlePermission::belonging_to(self)
            .filter(article_permissions::article_id.eq(article.id))
            .filter(article_permissions::can_edit.is_not_null())
            .first::<ArticlePermission>(conn)
            .optional()?;
        if let Some(article_permission) = article_permission {
            return Ok(article_permission.can_edit.unwrap());
        }

        let namespace_permission = NamespacePermission::belonging_to(self)
            .filter(namespace_permissions::namespace_id.eq(article.namespace_id))
            .first::<NamespacePermission>(conn)?;
        Ok(namespace_permission.can_edit)
    }

    pub fn can_rename(&self, conn: &PgConnection, article: &Article) -> Result<bool> {
        let article_permission = ArticlePermission::belonging_to(self)
            .filter(article_permissions::article_id.eq(article.id))
            .filter(article_permissions::can_rename.is_not_null())
            .first::<ArticlePermission>(conn)
            .optional()?;
        if let Some(article_permission) = article_permission {
            return Ok(article_permission.can_rename.unwrap());
        }

        let namespace_permission = NamespacePermission::belonging_to(self)
            .filter(namespace_permissions::namespace_id.eq(article.namespace_id))
            .first::<NamespacePermission>(conn)?;
        Ok(namespace_permission.can_rename)
    }

    pub fn can_delete(&self, conn: &PgConnection, article: &Article) -> Result<bool> {
        let article_permission = ArticlePermission::belonging_to(self)
            .filter(article_permissions::article_id.eq(article.id))
            .filter(article_permissions::can_delete.is_not_null())
            .first::<ArticlePermission>(conn)
            .optional()?;
        if let Some(article_permission) = article_permission {
            return Ok(article_permission.can_delete.unwrap());
        }

        let namespace_permission = NamespacePermission::belonging_to(self)
            .filter(namespace_permissions::namespace_id.eq(article.namespace_id))
            .first::<NamespacePermission>(conn)?;
        Ok(namespace_permission.can_delete)
    }

    pub fn can_create(&self, conn: &PgConnection, namespace: &Namespace) -> Result<bool> {
        let namespace_permission = NamespacePermission::belonging_to(self)
            .filter(namespace_permissions::namespace_id.eq(namespace.id))
            .first::<NamespacePermission>(conn)?;
        Ok(namespace_permission.can_create)
    }

    pub fn can_grant(&self, conn: &PgConnection, namespace: &Namespace) -> Result<bool> {
        let namespace_permission = NamespacePermission::belonging_to(self)
            .filter(namespace_permissions::namespace_id.eq(namespace.id))
            .first::<NamespacePermission>(conn)?;
        Ok(namespace_permission.can_grant)
    }
}
