#![allow(unused)]

use crate::model::article::ArticlePreview;
use crate::model::auth::{Admin, User};
use crate::{error::CustomError, AppState};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use shared::Article;

#[get("")]
pub async fn get_all(_: Admin, state: Data<AppState>) -> Result<Json<Vec<Article>>, CustomError> {
    let pool = &state.pool;

    let articles: Vec<Article> = sqlx::query_as("select * from articles")
        .fetch_all(pool)
        .await?;

    Ok(Json(articles))
}

#[get("/{id}")]
pub async fn get_by_id(
    id: web::Path<(u32,)>,
    state: Data<AppState>,
) -> Result<Json<Article>, CustomError> {
    let pool = &state.pool;

    let article: Article = sqlx::query_as("select * from articles where id = $1")
        .bind(id.0)
        .fetch_one(pool)
        .await?;

    Ok(Json(article))
}

#[post("/new")]
pub async fn new(
    state: Data<AppState>,
    Json(article): Json<Article>,
) -> Result<impl Responder, CustomError> {
    let pool = &state.pool;

    sqlx::query("insert into articles (title, content) values (?, ?)")
        .bind(article.title)
        .bind(article.content)
        .execute(pool)
        .await?;

    Ok(HttpResponse::Created().body("新增文章成功"))
}

#[put("/edit")]
pub async fn edit(
    state: Data<AppState>,
    Json(article): Json<Article>,
) -> Result<String, CustomError> {
    let pool = &state.pool;

    let id = article
        .id
        .ok_or(CustomError::BadRequest("请提供要修改的文章id".into()))?;

    sqlx::query("update articles set title = ?, content = ? where id = ?")
        .bind(article.title)
        .bind(article.content)
        .bind(id)
        .execute(pool)
        .await?;

    Ok("新增修改文章".into())
}

#[delete("/delete/{id}")]
pub async fn delete(id: web::Path<(u32,)>, state: Data<AppState>) -> Result<String, CustomError> {
    let pool = &state.pool;

    sqlx::query("delete from articles where id = ?")
        .bind(id.0)
        .execute(pool)
        .await?;

    Ok("删除文章成功".into())
}

#[get("/search/{keyword}")]
pub async fn serch(
    keyword: web::Path<(String,)>,
    state: Data<AppState>,
) -> Result<Json<Vec<ArticlePreview>>, CustomError> {
    let pool = &state.pool;

    let keyword = format!("%{}%", keyword.0);

    let result: Vec<ArticlePreview> =
        sqlx::query_as("select * from articles where title like $1 or content like $1")
            .bind(keyword)
            .fetch_all(pool)
            .await?;

    if result.is_empty() {
        Err(CustomError::NotFound("找不到文章".into()))
    } else {
        Ok(Json(result))
    }
}
