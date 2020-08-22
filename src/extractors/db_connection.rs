use actix_web::{dev, web::Data, Error, FromRequest, HttpRequest};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use futures::future::{ok, Ready};
use std::ops::Deref;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub conn: Conn,
}

impl Deref for DbConnection {
    type Target = Conn;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl FromRequest for DbConnection {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let pool = req.app_data::<Data<DbPool>>().unwrap();
        let conn = pool.get().unwrap();
        ok(DbConnection { conn })
    }
}
