use actix_web::http::header::ContentType;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

#[allow(dead_code)]
#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "time out")]
    Timeout,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "server database error")]
    DatabaseError(sqlx::Error),

    #[display(fmt = "internal server error")]
    InternalError,
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
            Self::BadClientData => StatusCode::BAD_REQUEST,
            Self::InternalError | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
