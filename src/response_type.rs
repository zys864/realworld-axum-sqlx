use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub token: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthResponse {
    pub user: User,
}
#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub user: Profile,
}

/// Article
///
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Article {
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
    pub author: Profile,
}
#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    article: Article,
}
#[derive(Debug, Serialize)]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    pub article_count: u32,
}
