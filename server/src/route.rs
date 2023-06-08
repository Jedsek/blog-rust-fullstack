use crate::api::*;
use actix_web::web::{self, ServiceConfig};

pub fn route(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .service(article::get_all)
            .service(article::get_by_id)
            .service(article::serch)
            .service(article::delete)
            .service(article::edit)
            .service(article::new),
    )
    .service(
        web::scope("/comment")
            .service(comment::get_all_for_article)
            .service(comment::new),
    )
    .service(web::scope("/user").service(user::github_login))
    .service(home);
}
