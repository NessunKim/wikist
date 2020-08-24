use crate::models::User;
use crate::schema::{roles, user_roles};
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Associations, Identifiable, Queryable, Debug)]
#[primary_key(user_id, role_id)]
#[belongs_to(User)]
#[belongs_to(Role)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
}
