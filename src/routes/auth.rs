use super::{Response, ResponseResult};
use crate::db;
use actix_web::{get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FbAuthRequest {
    access_token: String,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FbResponse {
    id: String,
}

#[post("/auth/facebook")]
pub async fn auth_facebook(
    pool: web::Data<db::DbPool>,
    fb_auth_request: web::Json<FbAuthRequest>,
) -> Result<HttpResponse, Error> {
    use reqwest::Url;
    let url = Url::parse(
        format!(
            "https://graph.facebook.com/me?fields=id,name&access_token={}",
            fb_auth_request.access_token
        )
        .as_str(),
    )
    .unwrap();
    let resp = reqwest::get(url)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?
        .json::<FbResponse>()
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?;
    if resp.id != fb_auth_request.user_id {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    Ok(HttpResponse::Created().finish())
}
