use anyhow::Result;
use axum::{Json, extract::Extension};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::response_type;
#[derive(Debug, Deserialize, Validate)]
pub struct UserCreate {
    #[validate(length(min = 6, max = 20))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 9, max = 20))]
    password: String,
}
#[derive(Debug, Deserialize)]
pub struct UserCreateRequest{
    user:UserCreate
}

pub async fn create_user(
    Json(user_create_request): Json<UserCreateRequest>,
    Extension(db):Extension<PgPool>
) -> Result<response_type::UserAuthResponse> {
    let user_create = user_create_request.user;

    let res = user_create.validate()?;
}
