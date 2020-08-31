use crate::models::{Article, Namespace, Role};
use crate::schema::{article_permissions, namespace_permissions};
use anyhow::Result;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Associations, Identifiable, Insertable, Queryable, Debug)]
#[primary_key(article_id, role_id)]
#[belongs_to(Article)]
#[belongs_to(Role)]
pub struct ArticlePermission {
    pub article_id: i32,
    pub role_id: i32,
    pub can_read: bool,
    pub can_edit: bool,
    pub can_rename: bool,
    pub can_delete: bool,
}

#[derive(Serialize, Associations, Identifiable, Insertable, Queryable, Debug)]
#[primary_key(namespace_id, role_id)]
#[belongs_to(Namespace)]
#[belongs_to(Role)]
pub struct NamespacePermission {
    pub namespace_id: i32,
    pub role_id: i32,
    pub can_read: bool,
    pub can_edit: bool,
    pub can_rename: bool,
    pub can_delete: bool,
}
