use crate::auth;
use crate::models::{Article, Namespace, Role, UserRole};
use crate::schema::{authentications, users};
use anyhow::Result;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Associations, Queryable, Identifiable, Debug)]
#[belongs_to(User)]
pub struct Authentication {
    pub id: i32,
    pub user_id: i32,
    pub provider: String,
    pub provider_user_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "authentications"]
pub struct NewAuthentication<'a> {
    pub user_id: i32,
    pub provider: &'a str,
    pub provider_user_id: &'a str,
    pub created_at: NaiveDateTime,
}

pub enum UserFindResult {
    Exists(User),
    WrongProvider(User), // Same email, but different provider
    NotExists,
}

/// Generates permission checking methods
macro_rules! permission_checker_for_article {
    ($x:ident) => {
        pub fn $x(&self, conn: &PgConnection, article: &Article) -> Result<bool> {
            for role in self.get_roles(conn)?.iter() {
                if role.$x(conn, article)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    };
}

/// Generates permission checking methods
macro_rules! permission_checker_for_namespace {
    ($x:ident) => {
        pub fn $x(&self, conn: &PgConnection, namespace: &Namespace) -> Result<bool> {
            for role in self.get_roles(conn)?.iter() {
                if role.$x(conn, namespace)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    };
}

impl User {
    pub fn find(
        conn: &PgConnection,
        email: &str,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<UserFindResult> {
        let auth = authentications::table
            .filter(authentications::provider.eq(provider))
            .filter(authentications::provider_user_id.eq(provider_user_id))
            .first::<Authentication>(conn)
            .optional()?;
        if let Some(auth) = auth {
            let user = auth.get_user(conn)?;
            return Ok(UserFindResult::Exists(user));
        }
        let user_with_same_email = users::table
            .filter(users::email.eq(email))
            .first::<User>(conn)
            .optional()?;
        if let Some(user_with_same_email) = user_with_same_email {
            return Ok(UserFindResult::WrongProvider(user_with_same_email));
        }
        Ok(UserFindResult::NotExists)
    }

    pub fn create(conn: &PgConnection, email: &str, username: &str) -> Result<Self> {
        let now = Utc::now().naive_utc();
        let new_user = NewUser {
            email,
            username,
            created_at: now,
            updated_at: now,
        };
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<Self>(conn)?;
        Ok(user)
    }

    pub fn add_authentication(
        &self,
        conn: &PgConnection,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Authentication> {
        let now = Utc::now().naive_utc();
        let new_auth = NewAuthentication {
            user_id: self.id,
            provider,
            provider_user_id,
            created_at: now,
        };
        let auth = diesel::insert_into(authentications::table)
            .values(new_auth)
            .get_result::<Authentication>(conn)?;
        Ok(auth)
    }

    pub fn issue_refresh_token(&self) -> String {
        use chrono::Duration;
        use jsonwebtoken::{encode, EncodingKey, Header};
        use std::env;
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let claims = auth::TokenClaims::new(
            self.id,
            Utc::now(),
            Utc::now() + Duration::days(30),
            "refresh",
        );

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .expect("JWT encoding failed")
    }

    pub fn add_role(&self, conn: &PgConnection, role: &Role) -> Result<()> {
        use crate::schema::user_roles;
        let user_role = UserRole {
            user_id: self.id,
            role_id: role.id,
        };
        user_role.insert_into(user_roles::table).execute(conn)?;
        Ok(())
    }

    pub fn get_roles(&self, conn: &PgConnection) -> Result<Vec<Role>> {
        use crate::schema::{roles, user_roles};
        let res = roles::table
            .inner_join(user_roles::table.inner_join(users::table))
            .filter(users::id.eq(self.id))
            .load::<(Role, (UserRole, User))>(conn)?;
        Ok(res.iter().map(|r| r.0.clone()).collect::<Vec<Role>>())
    }

    pub fn has_any_role(&self, conn: &PgConnection, roles: &[Role]) -> Result<bool> {
        use crate::schema::user_roles;
        let user_role = user_roles::table
            .filter(user_roles::user_id.eq(self.id))
            .filter(user_roles::role_id.eq_any(roles.iter().map(|r| r.id).collect::<Vec<i32>>()))
            .first::<UserRole>(conn)
            .optional()?;
        Ok(user_role.is_some())
    }

    permission_checker_for_article!(can_read);
    permission_checker_for_article!(can_edit);
    permission_checker_for_article!(can_rename);
    permission_checker_for_article!(can_delete);
    permission_checker_for_namespace!(can_create);
    permission_checker_for_namespace!(can_grant);
}

impl Authentication {
    pub fn get_user(&self, conn: &PgConnection) -> Result<User> {
        let user = users::table.find(self.user_id).get_result::<User>(conn)?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_connection;

    #[test]
    fn test_has_any_role() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let user = User::create(&conn, "test@testtest.com", "tester22").expect("must succeed");
            let role1 = Role::create(&conn, "Test Role").expect("must succeed");
            let roles = [role1];
            let has = user.has_any_role(&conn, &roles).expect("must succeed");
            assert_eq!(has, false);
            user.add_role(&conn, &roles[0]).expect("must succeed");
            let has = user.has_any_role(&conn, &roles).expect("must succeed");
            assert_eq!(has, true);
            Ok(())
        });
    }

    #[test]
    fn test_can_create_default() {
        let conn = create_connection();
        conn.test_transaction::<_, diesel::result::Error, _>(|| {
            let user = User::create(&conn, "test@testtest.com", "tester22").expect("must succeed");
            let role = Role::logged_in();
            user.add_role(&conn, &role).expect("must succeed");
            let can_create = user
                .can_create(&conn, &Namespace::default())
                .expect("must succeed");
            assert_eq!(can_create, true);
            Ok(())
        });
    }
}
