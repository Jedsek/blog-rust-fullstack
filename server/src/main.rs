#![allow(unused)]

mod api;
mod db;
mod error;

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpServer,
};
use db::Pool;
use error::CustomError;
use std::env;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub pool: Mutex<Pool>,
}

#[actix_web::main]
async fn main() -> Result<(), CustomError> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenvy::dotenv().expect("Failed to init .env file");
    pretty_env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");

    let state = web::Data::new(AppState {
        pool: Mutex::new(db::create_pool(&db_url).await?),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .service(api::home)
            .service(api::get_articles)
            .service(api::new_article)
            .service(api::edit_article)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn index() -> Result<&'static str, CustomError> {
    Ok("123")
}
