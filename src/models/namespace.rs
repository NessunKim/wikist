use crate::schema::namespaces;
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
