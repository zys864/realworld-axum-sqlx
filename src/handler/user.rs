use crate::{
    db::{hash, verify_hash},
    error::ErrorKind,
    jwt::{generate_jwt_token, Claims},
};
use axum::{extract::Extension, Json};
use axum_debug::debug_handler;
use serde::{Deserialize, Serialize};
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
    user_create.validate()?;

    // determine that the email is already registed or not
    let res = sqlx::query!(
        r#"
    SELECT email FROM user_info
    WHERE email = $1
    "#,
        user_create.email
    )
    .fetch_optional(&db)
    .await?;
    tracing::debug!("email featch result {:#?}", res);
    if let Some(record) = res {
        return Err(ErrorKind::DuplicatedEmail(record.email));
    }
    // hash password
    let hashed_password = hash(user_create.password)?;
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
    .await?;
    let claims = crate::jwt::Claims::new(user_create.email.clone());
    let token = generate_jwt_token(claims)?;
    let user_info = response_type::User {
        username: user_create.username,
        token: token,
        email: user_create.email,
        bio: None,
        image: None,
    };
    let user_response = response_type::UserAuthResponse { user: user_info };
    Ok(Json(user_response))
}

/// Login user
///
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUserResquest {
    pub user: LoginUser,
}

#[debug_handler]
pub async fn login_user(
    Json(user_login): Json<LoginUserResquest>,
    Extension(db): Extension<PgPool>,
) -> IResult<Json<response_type::UserAuthResponse>> {
    let LoginUser { email, password } = user_login.user;

    let res = sqlx::query!(
        r#"
        SELECT email,username,hashed_password FROM user_info
        WHERE email = $1
        "#,
        &email
    )
    .fetch_optional(&db)
    .await?;
    match res {
        Some(record) => {
            let is_auth = verify_hash(password, record.hashed_password)
                .map_err(|_| ErrorKind::Unauthorized)?;
            if is_auth {
                let claims = crate::jwt::Claims::new(record.email.clone());
                let token = generate_jwt_token(claims)?;
                let user_info = response_type::User {
                    username: record.username,
                    token: token,
                    email: record.email,
                    bio: None,
                    image: None,
                };
                let user_response = response_type::UserAuthResponse { user: user_info };
                Ok(Json(user_response))
            } else {
                Err(ErrorKind::Unauthorized)
            }
        }
        None => Err(ErrorKind::Unauthorized),
    }
}
#[debug_handler]
pub async fn get_current_user(
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<response_type::ProfileResponse>, ErrorKind> {
    let user_email = claims.sub; // email
    tracing::debug!("{}", &user_email);
    let user = sqlx::query_as!(
        response_type::Profile,
        r#"
        SELECT email,username,bio,image
        FROM user_info
        WHERE email=$1
        "#,
        user_email,
    )
    .fetch_one(&db)
    .await?;
    let user_response = response_type::ProfileResponse { user };
    Ok(Json(user_response))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 6, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 9, max = 20))]
    pub password: String,
    #[validate(length(min = 1, max = 20))]
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user: UpdateUser,
}

#[debug_handler]
pub async fn update_user(
    _claims: Claims,
    Json(update_user_info): Json<UpdateUserRequest>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<response_type::UserAuthResponse>, ErrorKind> {
    let update_user = update_user_info.user;
    // determine user create info validation
    update_user.validate()?;

    // hash password
    let hashed_password = hash(update_user.password)?;
    // insert into sql
    let _id = sqlx::query!(
        r#"
            INSERT INTO user_info(username,email,hashed_password,bio,image)
            VALUES ($1,$2,$3,$4,$5)
            RETURNING id
        "#,
        update_user.username,
        update_user.email,
        hashed_password,
        update_user.bio,
        update_user.image
    )
    .fetch_one(&db)
    .await?;
    let claims = crate::jwt::Claims::new(update_user.email.clone());
    let token = generate_jwt_token(claims)?;
    let user_info = response_type::User {
        username: update_user.username,
        token: token,
        email: update_user.email,
        bio: None,
        image: None,
    };
    let user_response = response_type::UserAuthResponse { user: user_info };
    Ok(Json(user_response))
}
