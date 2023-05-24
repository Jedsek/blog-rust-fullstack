mod errors;

use actix_web::{get, middleware, App, HttpServer};
use errors::CustomError;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(error)
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

#[get("/error")]
async fn error() -> Result<&'static str, CustomError> {
    Err(CustomError::NotFound)
}
