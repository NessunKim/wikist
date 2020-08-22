use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

pub struct UserInfo {
    pub id: i32,
}

impl FromRequest for UserInfo {
    type Error = Error;
    type Future = Ready<Result<UserInfo, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        match auth_header {
            Some(auth_header) => {
                let split: Vec<&str> = auth_header.to_str().unwrap().split("Bearer").collect();
                let token = split[1].trim();
                match crate::auth::decode(token) {
                    Ok(decoded) => ok(UserInfo {
                        id: decoded.claims.sub,
                    }),
                    Err(_e) => err(ErrorUnauthorized("invalid token!")),
                }
            }
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}
