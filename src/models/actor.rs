use crate::schema::actors;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Actor {
    pub id: i32,
    pub user_id: Option<i32>,
    pub ip_address: Option<IpNetwork>,
}

#[derive(Insertable)]
#[table_name = "actors"]
struct NewActor {
    pub user_id: Option<i32>,
    pub ip_address: Option<IpNetwork>,
}

impl Actor {
    pub fn find_or_create_from_user_id(conn: &PgConnection, user_id: i32) -> Result<Self> {
        use diesel::result::{DatabaseErrorKind, Error};
        let actor = actors::table
            .filter(actors::user_id.eq(user_id))
            .first::<Actor>(conn)
            .optional()?;
        match actor {
            Some(actor) => return Ok(actor),
            None => {}
        };
        let result = diesel::insert_into(actors::table)
            .values(NewActor {
                user_id: Some(user_id),
                ip_address: None,
            })
            .get_result(conn);
        match result {
            Ok(result) => Ok(result),
            // handle race condition
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                let actor = actors::table
                    .filter(actors::user_id.eq(user_id))
                    .first::<Actor>(conn)?;
                Ok(actor)
            }
            Err(e) => Err(anyhow!(e)),
        }
    }
    pub fn find_or_create_from_ip(conn: &PgConnection, ip_address: &IpNetwork) -> Result<Self> {
        use diesel::result::{DatabaseErrorKind, Error};
        let actor = actors::table
            .filter(actors::ip_address.eq(ip_address))
            .first::<Actor>(conn)
            .optional()?;
        match actor {
            Some(actor) => return Ok(actor),
            None => {}
        };
        let result = diesel::insert_into(actors::table)
            .values(NewActor {
                user_id: None,
                ip_address: Some(*ip_address),
            })
            .get_result(conn);
        match result {
            Ok(result) => Ok(result),
            // handle race condition
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                let actor = actors::table
                    .filter(actors::ip_address.eq(ip_address))
                    .first::<Actor>(conn)?;
                Ok(actor)
            }
            Err(e) => Err(anyhow!(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_connection;

    #[actix_rt::test]
    async fn test_find_or_create_from_ip_race_condition_1() {
        use ipnetwork::IpNetwork;
        use std::str::FromStr;
        let conn = create_connection();
        Actor::find_or_create_from_ip(
            &conn,
            &IpNetwork::from_str("123.12.3.4").expect("must succeed"),
        )
        .expect("must succeed");
    }

    #[actix_rt::test]
    async fn test_find_or_create_from_ip_race_condition_2() {
        use ipnetwork::IpNetwork;
        use std::str::FromStr;
        let conn = create_connection();
        Actor::find_or_create_from_ip(
            &conn,
            &IpNetwork::from_str("123.12.3.4").expect("must succeed"),
        )
        .expect("must succeed");
    }
}
