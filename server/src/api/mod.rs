mod article;

use crate::error::CustomError;
use actix_web::get;
use futures::io;

pub use article::*;

#[get("/")]
pub async fn home() -> &'static str {
    "Home page"
}
