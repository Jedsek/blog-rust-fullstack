use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::io;

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("not found")]
    NotFound(String),

    #[error("time out")]
    Timeout,

    #[error("bad request")]
    BadRequest(String),

    #[error("internal server error: {0}")]
    InternalError(String),
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

trait MyError: ToString {}

impl MyError for actix_web::Error {}
impl MyError for sqlx::Error {}
impl MyError for io::Error {}

impl<T> From<T> for CustomError
where
    T: MyError + ToString,
{
    fn from(err: T) -> Self {
        Self::InternalError(err.to_string())
    }
}
