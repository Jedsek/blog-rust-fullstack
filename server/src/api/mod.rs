use actix_web::{get, Responder};

pub mod article;
pub mod comment;
pub mod user;

#[get("/")]
pub async fn home() -> impl Responder {
    "Home page"
}
