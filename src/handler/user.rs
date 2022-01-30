use crate::{db::hash, error::ErrorKind};
use axum::{extract::Extension, Json};
use axum_debug::debug_handler;
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{error::IResult, response_type};
#[derive(Debug, Deserialize, Validate)]
pub struct UserCreate {
    #[validate(length(min = 6, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 9, max = 20))]
    pub password: String,
}
#[derive(Debug, Deserialize)]
pub struct UserCreateRequest {
    user: UserCreate,
}
#[debug_handler]
pub async fn create_user(
    Json(user_create_request): Json<UserCreateRequest>,
    Extension(db): Extension<PgPool>,
) -> IResult<Json<response_type::UserAuthResponse>> {
    let user_create = user_create_request.user;
    // determine user create info validation
    if let Err(e) = user_create.validate() {
        let error_info: Vec<String> =
            e.to_string().split("\n").map(|x| x.to_string()).collect();
        return Err(ErrorKind::filed_validate(error_info));
    }
    // determine that the email is already registed or not
    let res = sqlx::query!(
        r#"
    SELECT email FROM user_info
    WHERE email = $1
    "#,
        user_create.email
    )
    .fetch_optional(&db)
    .await
    .map_err(|e| ErrorKind::SqlError(e))?;
    tracing::debug!("email featch result {:#?}", res);
    if let Some(record) = res
    {
        return Err(ErrorKind::FiledValidate(vec![
            format!("email:{} is already in use",record.email)
        ]));
    }
    // hash password
    let hashed_password = hash(user_create.password)
        .map_err(|_| ErrorKind::SqlError(sqlx::Error::WorkerCrashed))?;
    tracing::debug!("hashed password:{:#?}", hashed_password);
    // insert into sql
    let _id = sqlx::query!(
        r#"
    INSERT INTO user_info(username,email,hashed_password)
    VALUES ($1,$2,$3)
    RETURNING id
    "#,
        &user_create.username,
        &user_create.email,
        &hashed_password
    )
    .fetch_one(&db)
    .await
    .map_err(|e| ErrorKind::SqlError(e))?;
    tracing::debug!("insert return id: {:#?}", &_id);
    let user_info = response_type::User{ username: user_create.username,
        email: user_create.email, bio: None, image: None };
    tracing::debug!("return user info:{:#?}", &user_info);
    let user_response = response_type::UserAuthResponse{
        user:user_info
    };
    Ok(Json(user_response))
}
