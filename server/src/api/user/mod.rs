mod login;

use crate::error::CustomError;
use crate::AppState;
use actix_web::web::Json;
use actix_web::{
    get,
    web::{self, Data},
};
pub use login::*;
use shared::users::GithubUserInfo;

// 63626406

#[get("/{id}")]
pub async fn get_by_id(
    id: web::Path<(u32,)>,
    state: Data<AppState>,
) -> Result<Json<GithubUserInfo>, CustomError> {
    let pool = &state.pool;

    let user_info: GithubUserInfo = sqlx::query_as("select * from users where id = $1")
        .bind(id.0)
        .fetch_one(pool)
        .await?;

    Ok(Json(user_info))
}
