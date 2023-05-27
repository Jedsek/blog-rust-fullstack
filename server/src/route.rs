use crate::api::*;
use actix_web::web::{self, ServiceConfig};

pub fn route(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .service(article::get_all)
            .service(article::get_one)
            .service(article::serch)
            .service(article::delete)
            .service(article::edit)
            .service(article::new),
    )
    .service(home);
}
