use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
