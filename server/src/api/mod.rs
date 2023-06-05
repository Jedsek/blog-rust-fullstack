use actix_web::get;

pub mod article;
pub mod user;

#[get("/")]
pub async fn home() -> &'static str {
    "Home page"
}
