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
    #[error("Duplicated email: {}", 0)]
    DuplicatedEmail(String),
    #[error(transparent)]
    FiledValidate(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
    #[error(transparent)]
    EncripyError(#[from] argon2::Error),
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
            ErrorKind::DuplicatedEmail(s) => {
                let errors_info = vec![format!("Duplicated email: {}", s)];
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
            ErrorKind::FiledValidate(e) => {
                let errors_info: Vec<String> =
                    e.to_string().split("\n").map(|x| x.to_string()).collect();
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
            ErrorKind::SqlError(_) | ErrorKind::EncripyError(_) => {
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
