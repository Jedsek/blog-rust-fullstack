mod api;
mod db;
mod error;
mod model;
mod route;

use actix_web::{middleware, web::Data, App, HttpServer};
use db::Pool;
use error::CustomError;
use std::env;

const URL: &str = "127.0.0.1:9090";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.3";

#[derive(Debug)]
pub struct AppState {
    pub pool: Pool,
}

#[actix_web::main]
async fn main() -> Result<(), CustomError> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenvy::dotenv().expect("Failed to init .env file");
    pretty_env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    let pool = db::create_pool(&db_url).await?;

    let state = Data::new(AppState { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(route::route)
            .wrap(middleware::Logger::default())
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allow_any_method()
                    .allow_any_header(),
            )
    })
    .bind(URL)?
    .run()
    .await?;

    Ok(())
}
