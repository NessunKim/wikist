use super::{Response, ResponseResult};
use crate::db;
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FbAuthRequest {
    access_token: String,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FbGraphResponse {
    id: String,
    name: String,
    email: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    refresh_token: String,
}

#[post("/auth/refresh")]
pub async fn refresh(refresh_request: web::Json<RefreshRequest>) -> Result<HttpResponse, Error> {
    use crate::auth::TokenClaims;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use std::env;
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    match decode::<TokenClaims>(
        &refresh_request.refresh_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => {
            let claims = TokenClaims::new(
                token.claims.sub,
                Utc::now(),
                Utc::now() + Duration::hours(1),
                "access",
            );
            let access_token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .expect("JWT encoding failed");
            let resp = Response {
                status: "OK".to_owned(),
                result: ResponseResult::Refresh { access_token },
            };
            Ok(HttpResponse::Ok().json(resp))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[post("/auth/facebook")]
pub async fn auth_facebook(
    pool: web::Data<db::DbPool>,
    fb_auth_request: web::Json<FbAuthRequest>,
) -> Result<HttpResponse, Error> {
    use crate::models::user::{create_user, find_user, UserFindResult};
    use anyhow::Error;
    use reqwest::Url;
    let url = Url::parse(
        format!(
            "https://graph.facebook.com/me?fields=id,name,email&access_token={}",
            fb_auth_request.access_token
        )
        .as_str(),
    )
    .unwrap();
    let fb_resp = reqwest::get(url)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?
        .json::<FbGraphResponse>()
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?;
    let email = match fb_resp.email {
        Some(email) => email,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };
    if fb_resp.id != fb_auth_request.user_id {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let name = fb_resp.name;
    let conn = pool.get().expect("couldn't get db connection from pool");
    let mut is_new_user: bool = false;
    let refresh_token: String = conn
        .transaction::<_, Error, _>(|| {
            let user_find_result = find_user(&conn, &email, "facebook", &fb_auth_request.user_id)?;
            let refresh_token = match user_find_result {
                UserFindResult::Exists(user) => {
                    is_new_user = false;
                    user.issue_refresh_token()
                }
                UserFindResult::WrongProvider(_) => {
                    return Err(anyhow!("User with same email already exists"));
                }
                UserFindResult::NotExists => {
                    is_new_user = true;
                    let user = create_user(&conn, &email, &name)?;
                    user.add_authentication(&conn, "facebook", &fb_auth_request.user_id)?;
                    user.issue_refresh_token()
                }
            };
            Ok(refresh_token)
        })
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?;
    let resp = Response {
        status: "OK".to_owned(),
        result: ResponseResult::Auth { refresh_token },
    };
    if is_new_user {
        Ok(HttpResponse::Created().json(resp))
    } else {
        Ok(HttpResponse::Ok().json(resp))
    }
}
