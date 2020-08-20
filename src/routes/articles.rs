use super::{Response, ResponseResult};
use crate::db;
use crate::middlewares::auth::UserInfo;
use crate::parser;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use actix_web_validator::ValidatedJson;
use anyhow::Result;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::Validate;

#[get("/articles/{full_title}")]
pub async fn get_by_full_title(
    pool: web::Data<db::DbPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    use crate::models::Article;
    let full_title = path.0.clone();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let article = match Article::find_by_full_title(&conn, &full_title) {
        Ok(Some(article)) => article,
        Ok(None) => {
            let full_title = path.0.clone();
            return Ok(HttpResponse::NotFound()
                .body(format!("No article found with full title: {}", &full_title)));
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let revision = match article.get_latest_revision(&conn) {
        Ok(revision) => revision,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let wikitext = match revision.get_wikitext(&conn) {
        Ok(wikitext) => wikitext,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let html = web::block(move || -> Result<String> {
        let parsed = parser::parse(&wikitext);
        let html = crate::renderer::render(&parsed);
        Ok(html)
    })
    .await
    .unwrap();
    let resp = Response {
        status: "OK".to_owned(),
        result: ResponseResult::ArticleGet {
            full_title: article.title,
            html,
        },
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ArticleCreateRequest {
    #[validate(length(min = 1, max = 300))]
    full_title: String,
    #[validate(length(min = 1, max = 1000000))]
    wikitext: String,
}

#[post("/articles")]
pub async fn create_article(
    req: HttpRequest,
    user_info: Option<UserInfo>,
    pool: web::Data<db::DbPool>,
    data: ValidatedJson<ArticleCreateRequest>,
) -> Result<HttpResponse, Error> {
    use crate::models::{Actor, Article};
    let conn_info = req.connection_info();
    let remote = conn_info.remote().unwrap();
    let ip_address = IpNetwork::from_str(remote.split(':').collect::<Vec<&str>>()[0]).unwrap();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let actor = match user_info {
        Some(user_info) => {
            Actor::find_or_create_from_user_id(&conn, user_info.id).map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?
        }
        None => Actor::find_or_create_from_ip(&conn, &ip_address).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?,
    };
    web::block(move || Article::create(&conn, &data.full_title, &data.wikitext, &actor))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Created().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_create_article() {
        let pool = db::create_connection_pool();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(create_article)).await;
        let data = ArticleCreateRequest {
            full_title: "AA".to_string(),
            wikitext: "==AA==\nasdf".to_string(),
        };
        let req = test::TestRequest::post()
            .peer_addr("127.0.0.1:22342".parse().unwrap())
            .set_json(&data)
            .uri("/articles")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 201);
    }

    #[actix_rt::test]
    async fn test_create_article_validation() {
        let pool = db::create_connection_pool();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(create_article)).await;
        {
            let data = ArticleCreateRequest {
                full_title: "".to_string(),
                wikitext: "==AA==\nasdf".to_string(),
            };
            let req = test::TestRequest::post()
                .peer_addr("127.0.0.1:22342".parse().unwrap())
                .set_json(&data)
                .uri("/articles")
                .to_request();
            let resp = test::call_service(&mut app, req).await;
            assert_eq!(resp.status().as_u16(), 400);
        }
        {
            let data = ArticleCreateRequest {
                full_title: "asdfsdf".to_string(),
                wikitext: "".to_string(),
            };
            let req = test::TestRequest::post()
                .peer_addr("127.0.0.1:22342".parse().unwrap())
                .set_json(&data)
                .uri("/articles")
                .to_request();
            let resp = test::call_service(&mut app, req).await;
            assert_eq!(resp.status().as_u16(), 400);
        }
    }

    #[actix_rt::test]
    async fn test_get_by_full_title_non_existing() {
        let pool = db::create_connection_pool();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(get_by_full_title)).await;
        let req = test::TestRequest::get()
            .peer_addr("127.0.0.1:22342".parse().unwrap())
            .uri("/articles/non-existing")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 404);
    }

    #[actix_rt::test]
    async fn test_create_and_read_article() {
        let pool = db::create_connection_pool();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(create_article)
                .service(get_by_full_title),
        )
        .await;
        let data = ArticleCreateRequest {
            full_title: "title".to_string(),
            wikitext: "==AA==\nasdf".to_string(),
        };
        let req = test::TestRequest::post()
            .peer_addr("127.0.0.1:22342".parse().unwrap())
            .set_json(&data)
            .uri("/articles")
            .to_request();
        test::call_service(&mut app, req).await;
        let req = test::TestRequest::get().uri("/articles/title").to_request();
        let result: Response = test::read_response_json(&mut app, req).await;
        println!("{:#?}", result);
        assert_eq!(result.status, "OK");
        if let ResponseResult::ArticleGet { full_title, html } = result.result {
            assert_eq!(full_title, "title");
            assert_eq!(html, "<h2>AA</h2>\n<p>asdf</p>");
        } else {
            panic!();
        }
    }
}
