use super::Response;
use crate::extractors::{ConnectionInfo, DbConnection, Query, UserInfo};
use crate::parser;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use actix_web_validator::ValidatedJson;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use validator::Validate;

#[derive(Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ArticleGetQueryFields {
    Html,
    Wikitext,
    #[serde(other)]
    _Other,
}

#[derive(Deserialize)]
pub struct ArticleGetQuery {
    pub fields: HashSet<ArticleGetQueryFields>,
}

impl Default for ArticleGetQuery {
    fn default() -> Self {
        Self {
            fields: HashSet::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleGetResponse {
    full_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wikitext: Option<String>,
}

#[get("/articles/{full_title}")]
pub async fn get_article(
    path: web::Path<(String,)>,
    query: Option<Query<ArticleGetQuery>>,
    conn: DbConnection,
) -> Result<HttpResponse, Error> {
    use crate::models::Article;
    let ArticleGetQuery { fields } = &*query.unwrap_or_default();
    let full_title = path.0.clone();
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
    let html = if fields.contains(&ArticleGetQueryFields::Html) {
        let wikitext = wikitext.clone();
        let html = web::block(move || -> Result<String> {
            let parsed = parser::parse(&wikitext);
            let html = crate::renderer::render(&parsed);
            Ok(html)
        })
        .await
        .unwrap();
        Some(html)
    } else {
        None
    };
    let resp = Response {
        status: "OK".to_owned(),
        data: ArticleGetResponse {
            full_title: article.title,
            html,
            wikitext: if fields.contains(&ArticleGetQueryFields::Wikitext) {
                Some(wikitext)
            } else {
                None
            },
        },
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ActorEntity {
    #[serde(rename_all = "camelCase")]
    User { username: String },
    #[serde(rename_all = "camelCase")]
    Anonymous { ip_address: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleRevisionEntity {
    id: i32,
    created_at: NaiveDateTime,
    actor: ActorEntity,
    comment: String,
}

pub type ArticleRevisionsGetResponse = Vec<ArticleRevisionEntity>;

#[get("/articles/{full_title}/revisions")]
pub async fn get_revisions(
    path: web::Path<(String,)>,
    conn: DbConnection,
) -> Result<HttpResponse, Error> {
    use crate::models::{Actor, Article};
    let full_title = path.0.clone();
    let article = match Article::find_by_full_title(&conn, &full_title) {
        Ok(Some(article)) => article,
        Ok(None) => {
            let full_title = path.0.clone();
            return Ok(HttpResponse::NotFound()
                .body(format!("No article found with full title: {}", &full_title)));
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let revisions = match article.get_all_revisions(&conn) {
        Ok(revisions) => revisions,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let data: Vec<ArticleRevisionEntity> = revisions
        .iter()
        .map(|rev| {
            let actor = rev.get_actor(&conn)?;
            match actor {
                Actor {
                    user_id: Some(_), ..
                } => Ok(ArticleRevisionEntity {
                    id: rev.id,
                    created_at: rev.created_at,
                    actor: ActorEntity::User {
                        username: actor.get_user(&conn)?.username,
                    },
                    comment: rev.comment.clone(),
                }),
                Actor {
                    ip_address: Some(ip_address),
                    ..
                } => Ok(ArticleRevisionEntity {
                    id: rev.id,
                    created_at: rev.created_at,
                    actor: ActorEntity::Anonymous {
                        ip_address: ip_address.ip().to_string(),
                    },
                    comment: rev.comment.clone(),
                }),
                _ => Err(anyhow!("Both user_id and ip_address are null.")),
            }
        })
        .collect::<Result<Vec<ArticleRevisionEntity>, _>>()
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let resp = Response {
        status: "OK".to_owned(),
        data,
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleCreateRequest {
    #[validate(length(min = 1, max = 300))]
    full_title: String,
    #[validate(length(min = 0, max = 1000000))]
    wikitext: String,
    #[validate(length(min = 0, max = 1000))]
    comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleCreateResponse {
    full_title: String,
    revision_id: i32,
}

#[post("/articles")]
pub async fn create_article(
    ConnectionInfo { ip_address }: ConnectionInfo,
    user_info: Option<UserInfo>,
    conn: DbConnection,
    data: ValidatedJson<ArticleCreateRequest>,
) -> Result<HttpResponse, Error> {
    use crate::models::{Actor, Article};
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
    let article = web::block(move || {
        Article::create(
            &conn,
            &data.full_title,
            &data.wikitext,
            &data.comment,
            &actor,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let resp = Response {
        status: "OK".to_owned(),
        data: ArticleCreateResponse {
            full_title: article.title,
            revision_id: article.latest_revision_id,
        },
    };
    Ok(HttpResponse::Created().json(resp))
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleEditRequest {
    #[validate(length(min = 0, max = 1000000))]
    wikitext: String,
    #[validate(length(min = 0, max = 1000))]
    comment: String,
}

pub type ArticleEditResponse = ArticleCreateResponse;

#[put("/articles/{full_title}")]
pub async fn edit_article(
    ConnectionInfo { ip_address }: ConnectionInfo,
    user_info: Option<UserInfo>,
    conn: DbConnection,
    path: web::Path<(String,)>,
    data: ValidatedJson<ArticleEditRequest>,
) -> Result<HttpResponse, Error> {
    use crate::models::{Actor, Article};
    let full_title = path.0.clone();
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
    let mut article = match Article::find_by_full_title(&conn, &full_title) {
        Ok(Some(article)) => article,
        Ok(None) => {
            let full_title = path.0.clone();
            return Ok(HttpResponse::NotFound()
                .body(format!("No article found with full title: {}", &full_title)));
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let revision = match article.edit(&conn, &data.wikitext, &data.comment, &actor) {
        Ok(revision) => revision,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let resp = Response {
        status: "OK".to_owned(),
        data: ArticleEditResponse {
            full_title: article.title,
            revision_id: revision.id,
        },
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleRenameRequest {
    #[validate(length(min = 1, max = 300))]
    full_title: String,
    #[validate(length(min = 0, max = 1000))]
    comment: String,
}

pub type ArticleRenameResponse = ArticleCreateResponse;

#[put("/articles/{full_title}/full-title")]
pub async fn rename_article(
    ConnectionInfo { ip_address }: ConnectionInfo,
    user_info: Option<UserInfo>,
    conn: DbConnection,
    path: web::Path<(String,)>,
    data: ValidatedJson<ArticleRenameRequest>,
) -> Result<HttpResponse, Error> {
    use crate::models::{Actor, Article};
    let full_title = path.0.clone();
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
    let mut article = match Article::find_by_full_title(&conn, &full_title) {
        Ok(Some(article)) => article,
        Ok(None) => {
            let full_title = path.0.clone();
            return Ok(HttpResponse::NotFound()
                .body(format!("No article found with full title: {}", &full_title)));
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let revision = match article.rename(&conn, &data.full_title, &data.comment, &actor) {
        Ok(revision) => revision,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let resp = Response {
        status: "OK".to_owned(),
        data: ArticleRenameResponse {
            full_title: article.title,
            revision_id: revision.id,
        },
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleDeleteRequest {
    #[validate(length(min = 0, max = 1000))]
    comment: String,
}

pub type ArticleDeleteResponse = ArticleCreateResponse;

#[delete("/articles/{full_title}")]
pub async fn delete_article(
    ConnectionInfo { ip_address }: ConnectionInfo,
    user_info: Option<UserInfo>,
    conn: DbConnection,
    path: web::Path<(String,)>,
    data: ValidatedJson<ArticleRenameRequest>,
) -> Result<HttpResponse, Error> {
    use crate::models::{Actor, Article};
    let full_title = path.0.clone();
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
    let mut article = match Article::find_by_full_title(&conn, &full_title) {
        Ok(Some(article)) => article,
        Ok(None) => {
            let full_title = path.0.clone();
            return Ok(HttpResponse::NotFound()
                .body(format!("No article found with full title: {}", &full_title)));
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let revision = match article.delete(&conn, &data.comment, &actor) {
        Ok(revision) => revision,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let resp = Response {
        status: "OK".to_owned(),
        data: ArticleDeleteResponse {
            full_title: article.title,
            revision_id: revision.id,
        },
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_create_article() {
        let pool = db::create_connection_pool();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(create_article)).await;
        let data = ArticleCreateRequest {
            full_title: "AA".to_owned(),
            wikitext: "==AA==\nasdf".to_owned(),
            comment: "Comment!".to_owned(),
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
                full_title: "".to_owned(),
                wikitext: "==AA==\nasdf".to_owned(),
                comment: "Comment!".to_owned(),
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
                full_title: "asdfsdf".to_owned(),
                wikitext: "".to_owned(),
                comment: "Comment!".to_owned(),
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
    async fn test_get_article_non_existing() {
        let pool = db::create_connection_pool();
        let mut app = test::init_service(App::new().data(pool.clone()).service(get_article)).await;
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
                .service(get_article),
        )
        .await;
        let data = ArticleCreateRequest {
            full_title: "title".to_owned(),
            wikitext: "==AA==\nasdf".to_owned(),
            comment: "Comment!".to_owned(),
        };
        let req = test::TestRequest::post()
            .peer_addr("127.0.0.1:22342".parse().unwrap())
            .set_json(&data)
            .uri("/articles")
            .to_request();
        test::call_service(&mut app, req).await;
        let req = test::TestRequest::get()
            .uri("/articles/title?fields[]=html")
            .to_request();
        let result: Response<ArticleGetResponse> = test::read_response_json(&mut app, req).await;
        dbg!(&result);
        assert_eq!(result.status, "OK");
        let ArticleGetResponse {
            full_title,
            html,
            wikitext,
        } = result.data;
        assert_eq!(full_title, "title");
        assert_eq!(html, Some("<h2>AA</h2>\n<p>asdf</p>".to_owned()));
        assert_eq!(wikitext.is_none(), true);
    }
}
