use crate::db;
use actix_web::{get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[get("/articles/{full_title}")]
pub async fn get_by_full_title(
    pool: web::Data<db::DbPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let full_title = path.0.clone();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let article = web::block(move || db::get_article_by_full_title(&conn, &full_title))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(article) = article {
        Ok(HttpResponse::Ok().json(article))
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
pub async fn post(
    pool: web::Data<db::DbPool>,
    article: web::Json<ArticlePostData>,
) -> Result<HttpResponse, Error> {
    use db::models::NewArticle;
    let conn = pool.get().expect("couldn't get db connection from pool");
    web::block(move || {
        db::create_article(
            &conn,
            &NewArticle {
                title: &article.full_title,
                wikitext: &article.wikitext,
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
        let mut app = test::init_service(App::new().service(post)).await;
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
        let mut app = test::init_service(App::new().service(get_by_full_title)).await;
        let req = test::TestRequest::post()
            .uri("/articles/non-existing")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 404);
    }

    #[actix_rt::test]
    async fn test_get_by_full_title_existing() {
        let mut app = test::init_service(App::new().service(get_by_full_title)).await;
        let req = test::TestRequest::post()
            .uri("/articles/existing")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(test::read_body(resp).await, "existing!");
    }

    #[actix_rt::test]
    async fn test_create_and_read_article() {
        let mut app = test::init_service(App::new().service(post).service(get_by_full_title)).await;
        let data = ArticlePostData {
            full_title: "existing".to_string(),
            wikitext: "==AA==\nasdf".to_string(),
        };
        let req = test::TestRequest::post()
            .set_json(&data)
            .uri("/articles")
            .to_request();
        test::call_service(&mut app, req).await;
        let req = test::TestRequest::get()
            .uri("/articles/existing")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(test::read_body(resp).await, "existing!");
    }
}
