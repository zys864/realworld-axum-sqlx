use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub token:String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct UserAuthResponse {
    pub user: User,
}
