use axum::{
    extract::{Extension, Query, TypedHeader},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{auth::is_auth, error::ErrorKind, response_type};

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
pub async fn list_most_recent_articles(
    Query(req): Query<ArticleQuery>,
    TypedHeader(headers): TypedHeader<axum::headers::HeaderMap>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<response_type::ArticlesResponse>, ErrorKind> {
    req.validate()?;
    let is_auth = is_auth(headers).ok();
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
