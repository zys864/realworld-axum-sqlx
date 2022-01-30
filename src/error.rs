use axum::{
    body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http::header;
use serde::Serialize;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("not be authorized")]
    Unauthorized,
    #[error("token value error")]
    TokenError,
    #[error("token value expired")]
    TokenExpired,
    #[error("request json filed is invalided")]
    FiledValidate(Vec<String>),
    #[error("sqlx error")]
    SqlError(#[from] sqlx::Error),
}
impl ErrorKind {
    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }
    pub fn token_error() -> Self {
        Self::TokenError
    }
    pub fn token_expired() -> Self {
        Self::TokenExpired
    }
    pub fn filed_validate<T: AsRef<[String]>>(e: T) -> Self {
        Self::FiledValidate(e.as_ref().to_vec())
    }
    pub fn sql_error(e: sqlx::Error) -> Self {
        Self::SqlError(e)
    }
}
pub type IResult<T> = Result<T, ErrorKind>;
impl IntoResponse for ErrorKind {
    fn into_response(self) -> Response {
        match self {
            ErrorKind::Unauthorized => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(body::boxed(body::Empty::new()))
                .unwrap(),
            ErrorKind::TokenError => {
                let errors_info = vec!["TokenError".to_string()];
                let errors = ErrorRespinse::new(errors_info);
                Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .header(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )
                    .body(body::boxed(body::Full::from(
                        serde_json::to_string(&errors).unwrap(),
                    )))
                    .unwrap()
            }
            ErrorKind::TokenExpired => {
                let errors_info = vec!["TokenExpired".to_string()];
                let errors = ErrorRespinse::new(errors_info);
                Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .header(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )
                    .body(body::boxed(body::Full::from(
                        serde_json::to_string(&errors).unwrap(),
                    )))
                    .unwrap()
            }
            ErrorKind::FiledValidate(errors_info) => {
                let errors = ErrorRespinse::new(errors_info);
                Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .header(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )
                    .body(body::boxed(body::Full::from(
                        serde_json::to_string(&errors).unwrap(),
                    )))
                    .unwrap()
            }
            ErrorKind::SqlError(_) => {
                let errors_info = vec!["Internel server error".to_string()];
                let errors = ErrorRespinse::new(errors_info);
                Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .header(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str("application/json").unwrap(),
                    )
                    .body(body::boxed(body::Full::from(
                        serde_json::to_string(&errors).unwrap(),
                    )))
                    .unwrap()
            }
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ErrorRespinse {
    errors: ErrorResponseBody,
}
#[derive(Debug, Serialize)]
pub struct ErrorResponseBody {
    body: Vec<String>,
}
impl ErrorRespinse {
    pub fn new<T: AsRef<[String]>>(errors: T) -> Self {
        Self {
            errors: ErrorResponseBody {
                body: errors.as_ref().to_vec(),
            },
        }
    }
}
