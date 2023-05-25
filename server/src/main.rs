mod db;
mod error;

use actix_web::{get, middleware, App, HttpServer};
use error::CustomError;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), CustomError> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenvy::dotenv().expect("Failed to init .env file");
    pretty_env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    let _pool = db::create_pool(&db_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(error_1)
    })
    .bind("127.0.0.1:9090")
    .map_err(|_| CustomError::InternalError)?
    .run()
    .await
    .map_err(|_| CustomError::InternalError)?;

    Ok(())
}

#[get("/")]
async fn index() -> Result<&'static str, CustomError> {
    Ok("123")
}

#[get("/error")]
async fn error_1() -> Result<&'static str, CustomError> {
    Err(CustomError::NotFound)
}
