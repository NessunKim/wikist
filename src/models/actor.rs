use crate::auth;
use crate::schema::actors;
use anyhow::Result;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Actor {
    pub id: i32,
    pub user_id: Option<i32>,
    pub ip_address: Option<String>,
}
