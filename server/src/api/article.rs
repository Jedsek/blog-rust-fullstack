#![allow(unused)]

use crate::{error::CustomError, AppState};
use actix_web::{
    get, post,
    web::{self, Json},
};

#[get("/")]
pub async fn home() -> &'static str {
    "Home page"
}

#[get("/articles")]
pub async fn get_articles(
    data: web::Data<AppState>,
) -> Result<Json<Vec<common::articles::Article>>, CustomError> {
    todo!()
}
