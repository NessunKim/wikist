use crate::schema::namespaces;
use anyhow::Result;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Namespace {
    pub id: i32,
    pub name: String,
}

impl Default for Namespace {
    fn default() -> Self {
        Self {
            id: 1,
            name: "_DEFAULT".to_owned(),
        }
    }
}

impl Namespace {
    pub fn find_by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, diesel::result::Error> {
        if id == 1 {
            return Ok(Some(Self::default()));
        }
        let namespace = namespaces::table.find(id).first::<Self>(conn).optional()?;
        Ok(namespace)
    }
    pub fn find_by_name(
        conn: &PgConnection,
        name: &str,
    ) -> Result<Option<Self>, diesel::result::Error> {
        let namespace = namespaces::table
            .filter(namespaces::name.eq(name))
            .first::<Self>(conn)
            .optional()?;
        Ok(namespace)
    }
    pub fn parse_full_title(conn: &PgConnection, full_title: &str) -> Result<(Namespace, String)> {
        let full_title = full_title.trim();
        let split: Vec<&str> = full_title.splitn(2, ':').map(|s| s.trim()).collect();
        match split.as_slice() {
            [_first] => Ok((Self::default(), full_title.to_owned())),
            [first, second] => {
                if let Some(namespace) = Self::find_by_name(conn, first)? {
                    Ok((namespace, (*second).to_owned()))
                } else {
                    Ok((Self::default(), full_title.to_owned()))
                }
            }
            _ => panic!(),
        }
    }
}
