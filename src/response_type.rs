use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub token: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Profile {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthResponse {
    pub user: User,
}
#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub user: Profile,
}
