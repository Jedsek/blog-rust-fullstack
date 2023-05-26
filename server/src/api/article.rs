use crate::{error::CustomError, AppState};
use actix_web::{
    get, post, put,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use shared::Article;
use std::str::FromStr;

#[get("/articles")]
pub async fn get_articles(state: Data<AppState>) -> Result<Json<Vec<Article>>, CustomError> {
    let pool = &*state.pool.lock().await;

    let articles = sqlx::query_as!(
        Article,
        r#"select title, content, id as "id: Option<u32>", date as "date: Option<NaiveDate>" from articles"#
    )
    .fetch_all(pool)
    .await?;

    Ok(Json(articles))
}

#[post("/article")]
pub async fn new_article(
    state: Data<AppState>,
    article: Json<Article>,
) -> Result<impl Responder, CustomError> {
    let pool = &*state.pool.lock().await;

    sqlx::query!(
        "insert into articles (title, content) values (?, ?)",
        article.title,
        article.content
    )
    .execute(pool)
    .await?;

    Ok(HttpResponse::Created().body("新增文章成功"))
}

#[put("/article")]
pub async fn edit_article(
    state: Data<AppState>,
    article: Json<Article>,
) -> Result<String, CustomError> {
    let pool = &*state.pool.lock().await;

    let id = article
        .id
        .ok_or(CustomError::BadRequest("请提供要修改的文章id".into()))?;

    sqlx::query!(
        "update articles set title = ?, content = ? where id = ?",
        article.title,
        article.content,
        id,
    )
    .execute(pool)
    .await?;

    Ok("新增修改文章".into())
}
