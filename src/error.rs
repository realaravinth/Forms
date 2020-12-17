use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::ValidationErrors;

use std::convert::From;

#[derive(Debug, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[error("Looks like you've already submitted this form")] //405j
    DuplicateResponse,
    #[error("invalid credentials")]
    InternalServerError,
    #[error("The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[error("Response Doesn't exist")]
    BadRequest,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        actix_web::dev::HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::DuplicateResponse => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<actix_web::error::Error> for ServiceError {
    fn from(_: actix_web::error::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<ValidationErrors> for ServiceError {
    fn from(_: ValidationErrors) -> ServiceError {
        ServiceError::NotAnEmail
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::error::ServiceError>;
