use axum::{
    extract::{Extension, Query, TypedHeader, Path},
    Json, headers::authorization::Bearer,
};
// use axum_debug::debug_handler;
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{ PgPool};
use validator::Validate;

use crate::{
    auth::is_auth,
    error::ErrorKind,
    jwt::Claims,
    response_type,
};

// *****************************************************************************
#[derive(Debug, Deserialize, Validate)]
pub struct ArticleQuery {
    #[validate(length(min = 1, max = 20))]
    tag: Option<String>,
    #[validate(length(min = 6, max = 20))]
    author: Option<String>,
    favorited: Option<bool>,
    #[validate(range(min = 1, max = 200))]
    limit: Option<i64>,
    #[validate(range(min = 1))]
    offset: Option<i64>,
}
// flatten response_type::Article for pg return
#[derive(Debug, Deserialize)]
struct ArticleDbRerurn {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub body: String,
    #[serde(rename = "tagList")]
    pub taglist: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub createdat: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updatedat: DateTime<Utc>,
    pub favorited: bool,
    #[serde(rename = "favoritedsCount")]
    pub favoritescount: i32,
    pub username: String,
    pub following: bool,
    pub bio: Option<String>,
    pub image: Option<String>,
}
#[debug_handler]
pub async fn list_most_recent_articles(
    Query(req): Query<ArticleQuery>,
    TypedHeader(auth_info): TypedHeader<axum::headers::Authorization<Bearer>>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<response_type::ArticlesResponse>, ErrorKind> {
    req.validate()?;
    let is_auth = is_auth(auth_info).ok();
    let email = if let Some(claims) = is_auth {
        Some(claims.sub)
    } else {
        None
    };
    let auth_articles = sqlx::query_as!(ArticleDbRerurn,
    r#"
    SELECT slug as "slug!",title as "title!",description,body as "body!",
    taglist,createdat as "createdat!",updatedat as "updatedat!",favorited as "favorited!",
    favoritescount as "favoritescount!",username as "username!",following as "following!",bio,image
    FROM list_recent_article(
        (SELECT user_id FROM user_info WHERE email = $1)
        ,$2,$3,$4,$5,$6)
    "#,
    email,req.tag,req.author,req.favorited,req.limit,req.offset
    ).fetch_all(&db)
    .await?;

    let sql_articles: Vec<response_type::Article> = auth_articles
        .into_iter()
        .map(|article| {
            let profile = response_type::Profile {
                username: article.username,
                bio: article.bio,
                image: article.image,
                following: article.following,
            };
            response_type::Article {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                taglist: article.taglist,
                createdat: article.createdat,
                updatedat: article.updatedat,
                favorited: article.favorited,
                favoritescount: article.favoritescount,
                author: profile,
            }
        })
        .collect();
    let len = sql_articles.len() as u32;
    Ok(Json(response_type::ArticlesResponse {
        articles: sql_articles,
        article_count: len,
    }))
}
// *****************************************************************************

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    title: String,
    description: Option<String>,
    body: String,
    #[serde(rename = "tagList")]
    taglist: Vec<String>,
}
pub async fn create_article(
    Json(req): Json<CreateArticleRequest>,
    claims: Claims,
    Extension(db): Extension<PgPool>,
) -> Result<Json<response_type::ArticleResponse>, ErrorKind> {
    let email = claims.sub;
    let slug = req.title.replace(' ', "_");
    let user_info = sqlx::query!(
        r#"SELECT user_id,username,bio,image,True as following 
    FROM user_info
    WHERE user_info.email = $1
    "#,
        &email
    )
    .fetch_one(&db)
    .await?;
    let article = sqlx::query!(
        r#"
    INSERT INTO article (slug,title,description,body,taglist,author)
    VALUES ($1,$2,$3,$4,$5,$6)
    RETURNING slug,title,description,body,taglist,createdat,author
    "#,
        slug,
        req.title,
        req.description,
        req.body,
        &req.taglist,
        &user_info.user_id
    )
    .fetch_one(&db)
    .await?;
    let article_response = response_type::Article {
        slug,
        title: article.title,
        description: article.description,
        body: article.body,
        taglist: article.taglist,
        createdat: article.createdat,
        updatedat: article.createdat,
        favorited: true,
        favoritescount: 0,
        author: response_type::Profile {
            username: user_info.username,
            bio: user_info.bio,
            image: user_info.image,
            following: true,
        },
    };
    Ok(Json(response_type::ArticleResponse {
        article: article_response,
    }))
}
// *****************************************************************************


pub async fn get_article(
    Path(slug): Path<String>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<response_type::ArticleResponse>, ErrorKind> {
    let article = sqlx::query_as!(ArticleDbRerurn,
    r#"SELECT slug,title,description,body,taglist,createdat,updatedat,
    True as "favorited!",username,bio,image,true as "following!",
    (SELECT count(*) FROM favorites 
    WHERE favorites.article = article.article_id) as "favoritescount!:i32"
    FROM article,user_info
    WHERE article.slug=$1 and article.author=user_info.user_id
    "#,
        slug
    )
    .fetch_one(&db)
    .await?;
    
    let article_response = response_type::Article {
        slug,
        title: article.title,
        description: article.description,
        body: article.body,
        taglist: article.taglist,
        createdat: article.createdat,
        updatedat: article.createdat,
        favorited: article.favorited,
        favoritescount: article.favoritescount,
        author: response_type::Profile {
            username: article.username,
            bio: article.bio,
            image: article.image,
            following: article.following,
        },
    };
    Ok(Json(response_type::ArticleResponse {
        article: article_response,
    }))
}
