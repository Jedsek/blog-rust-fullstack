use crate::{
    error::CustomError,
    model::auth::{Admin, User},
    AppState,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use shared::Comment;

#[get("/{article_id}")]
pub async fn get_all_for_article(
    article_id: Path<(u32,)>,
    state: Data<AppState>,
) -> Result<Json<Vec<Comment>>, CustomError> {
    let pool = &state.pool;

    let comments: Vec<Comment> = sqlx::query_as("select * from comments where article_id = ?")
        .bind(article_id.0)
        .fetch_all(pool)
        .await?;

    Ok(Json(comments))
}

#[post("")]
pub async fn new(
    user: User,
    comment: Json<Comment>,
    state: Data<AppState>,
) -> Result<impl Responder, CustomError> {
    let pool = &state.pool;

    if sqlx::query("select * from articles where id = ?")
        .bind(comment.article_id)
        .fetch_optional(pool)
        .await?
        .is_none()
    {
        return Err(CustomError::BadRequest("要评论的文章不存在".into()));
    }

    sqlx::query("insert into comments (user_id, article_id, content) values (?, ?, ?)")
        .bind(user.id)
        .bind(comment.article_id)
        .bind(&comment.content)
        .execute(pool)
        .await?;

    Ok(HttpResponse::Created().body("新增评论成功"))
}

#[post("/{comment_id}")]
pub async fn delete(
    user: User,
    admin: Option<Admin>,
    comment_id: Path<(u32,)>,
    state: Data<AppState>,
) -> Result<impl Responder, CustomError> {
    let pool = &state.pool;
    let is_admin = admin.is_some();

    let rows_affected = if is_admin {
        sqlx::query("delete from comments where id = ?")
            .bind(comment_id.0)
            .execute(pool)
            .await?
    } else {
        sqlx::query("delete from comments where id = ? and user_id = ?")
            .bind(comment_id.0)
            .bind(user.id)
            .execute(pool)
            .await?
    }
    .rows_affected();

    if rows_affected != 0 {
        Ok("评论已被删除")
    } else {
        Err(CustomError::NotFound(
            "删除评论失败, 可能是提供的评论id不正确或者你没有权限删除此评论".into(),
        ))
    }
}
