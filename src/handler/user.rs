use std::sync::Arc;

use crate::{
    auth::{hash, verify_hash},
    error::ErrorKind,
    handler::AppState,
    utils::jwt::{generate_jwt_token, Claims},
};
use axum::debug_handler;
use axum::{
    extract::{State, TypedHeader},
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Result, response_type};
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
    State(state): State<Arc<AppState>>,
    Json(user_create_request): Json<UserCreateRequest>,
) -> Result<Json<response_type::UserAuthResponse>> {
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
    .fetch_optional(&state.db)
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
            RETURNING user_id
        "#,
        &user_create.username,
        &user_create.email,
        &hashed_password
    )
    .fetch_one(&state.db)
    .await?;
    let claims = Claims::new(user_create.email.clone());
    let token = generate_jwt_token(claims)?;
    let user_info = response_type::User {
        username: user_create.username,
        token,
        email: user_create.email,
        bio: None,
        image: None,
    };
    let user_response = response_type::UserAuthResponse { user: user_info };
    Ok(Json(user_response))
}
// *****************************************************************************
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
#[tracing::instrument(skip_all)]
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUserResquest>,
) -> Result<Json<response_type::UserAuthResponse>> {
    // Check if the user sent the credentials
    let LoginUser { email, password } = payload.user;
    tracing::info!("[user: {}],attempt login ", email);
    if email.is_empty() || password.is_empty() {
        tracing::info!(
            "[user: {}],failed to login, the email or password is empty",
            email
        );
        return Err(ErrorKind::Unauthorized);
    }

    // Here you can check the user credentials from a database
    let res = sqlx::query!(
        r#"
            SELECT email,username,hashed_password FROM user_info
            WHERE email = $1
            "#,
        &email
    )
    .fetch_optional(&state.db)
    .await?;
    match res {
        Some(record) => {
            let is_auth = verify_hash(password, record.hashed_password)
                .map_err(|_| ErrorKind::Unauthorized)?;
            if is_auth {
                let claims = Claims::new(record.email.clone());
                let token = generate_jwt_token(claims)?;
                let user_info = response_type::User {
                    username: record.username,
                    token,
                    email: record.email,
                    bio: None,
                    image: None,
                };
                let user_response = response_type::UserAuthResponse { user: user_info };
                tracing::info!("[user: {}],successful to login ", email);
                Ok(Json(user_response))
            } else {
                tracing::info!("user: {},failed to login ", email);
                Err(ErrorKind::Unauthorized)
            }
        }
        None => {
            tracing::info!("[user: {}],the user don't existed ", email);
            Err(ErrorKind::Unauthorized)
        }
    }
}

// *****************************************************************************

#[debug_handler]
pub async fn get_current_user(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    TypedHeader(bearer): TypedHeader<
        axum::headers::Authorization<axum::headers::authorization::Bearer>,
    >,
) -> Result<Json<response_type::UserAuthResponse>> {
    let token = bearer.token().to_string();
    let user_email = claims.sub; // email
    let record = sqlx::query!(
        r#"
        SELECT username,bio,image
        FROM user_info
        WHERE email=$1
        "#,
        &user_email,
    )
    .fetch_one(&state.db)
    .await?;
    let user = response_type::User {
        username: record.username,
        token,
        email: user_email,
        bio: record.bio,
        image: record.image,
    };
    let user_response = response_type::UserAuthResponse { user };
    Ok(Json(user_response))
}

// *****************************************************************************

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

    State(state): State<Arc<AppState>>,
    Json(update_user_info): Json<UpdateUserRequest>,
) -> Result<Json<response_type::UserAuthResponse>> {
    let update_user = update_user_info.user;
    // determine user create info validation
    update_user.validate()?;

    // hash password
    let hashed_password = hash(update_user.password)?;
    // insert into sql
    let _id = sqlx::query!(
        r#"
            UPDATE user_info SET username=$1,email=$2,hashed_password=$3,bio=$4,image=$5
        "#,
        update_user.username,
        update_user.email,
        hash(hashed_password)?,
        update_user.bio,
        update_user.image
    )
    .fetch_one(&state.db)
    .await?;
    let claims = Claims::new(update_user.email.clone());
    let token = generate_jwt_token(claims)?;
    let user_info = response_type::User {
        username: update_user.username,
        token,
        email: update_user.email,
        bio: None,
        image: None,
    };
    let user_response = response_type::UserAuthResponse { user: user_info };
    Ok(Json(user_response))
}
