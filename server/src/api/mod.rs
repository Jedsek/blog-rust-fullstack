use actix_web::get;

pub mod article;

#[get("/")]
pub async fn home() -> &'static str {
    "Home page"
}
