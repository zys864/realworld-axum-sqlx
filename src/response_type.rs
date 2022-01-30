use serde::Serialize;
use validator::Validate;

#[derive(Debug, Serialize, Validate)]
pub struct User {
    #[validate(length(min = 6, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 200))]
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct UserAuthResponse {
    pub user: User,
}
