use actix_web::{get, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
pub mod articles;
pub mod auth;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: String,
    result: ResponseResult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseResult {
    #[serde(rename_all = "camelCase")]
    ArticleGet {
        full_title: String,
        html: String,
    },
    #[serde(rename_all = "camelCase")]
    ArticleCreate {
        full_title: String,
        revision_id: i32,
    },
    #[serde(rename_all = "camelCase")]
    ArticleEdit {
        full_title: String,
        revision_id: i32,
    },
    #[serde(rename_all = "camelCase")]
    ArticleDelete {
        full_title: String,
        revision_id: i32,
    },
    #[serde(rename_all = "camelCase")]
    Auth {
        refresh_token: String,
        access_token: String,
    },
    #[serde(rename_all = "camelCase")]
    Refresh {
        access_token: String,
    },
    Hello,
}

#[get("/")]
pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_get() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::with_header("content-type", "text/plain")
            .uri("/")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(test::read_body(resp).await, "Hello!");
    }
}
