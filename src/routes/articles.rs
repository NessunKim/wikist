use super::{Response, ResponseResult};
use crate::db;
use crate::parser;
use actix_web::{get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[get("/articles/{full_title}")]
pub async fn get_by_full_title(
    pool: web::Data<db::DbPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    use crate::models::article;
    let full_title = path.0.clone();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let article = web::block(move || article::get_article_by_full_title(&conn, &full_title))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(article) = article {
        let result = parser::parse(&article.wikitext);
        let resp = Response {
            status: "OK".to_owned(),
            result: ResponseResult::ArticleGet {
                full_title: article.title,
                html: crate::renderer::render(&result),
            },
        };
        Ok(HttpResponse::Ok().json(resp))
    } else {
        let full_title = path.0.clone();
        let res = HttpResponse::NotFound()
            .body(format!("No article found with full title: {}", &full_title));
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticlePostData {
    full_title: String,
    wikitext: String,
}

#[post("/articles")]
pub async fn create_article(
    pool: web::Data<db::DbPool>,
    article: web::Json<ArticlePostData>,
) -> Result<HttpResponse, Error> {
    use crate::models::article;
    use article::NewArticle;
    let conn = pool.get().expect("couldn't get db connection from pool");
    let now = SystemTime::now();

    web::block(move || {
        article::create_article(
            &conn,
            &NewArticle {
                title: &article.full_title,
                wikitext: &article.wikitext,
                created_at: now,
                updated_at: now,
            },
        )
    })
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
    async fn test_post() {
        let pool = db::create_connection_pool();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(create_article)).await;
        let data = ArticlePostData {
            full_title: "AA".to_string(),
            wikitext: "==AA==\nasdf".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(&data)
            .uri("/articles")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 201);
    }

    #[actix_rt::test]
    async fn test_get_by_full_title_non_existing() {
        let pool = db::create_connection_pool();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(get_by_full_title)).await;
        let req = test::TestRequest::get()
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
        let data = ArticlePostData {
            full_title: "title".to_string(),
            wikitext: "==AA==\nasdf".to_string(),
        };
        let req = test::TestRequest::post()
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
