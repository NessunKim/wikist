use crate::models::User;
use crate::schema::{roles, user_roles};
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
}
