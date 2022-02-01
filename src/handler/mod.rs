use axum::{routing::post, Router};

pub mod user;
pub fn app() -> Router {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users/login", post(user::login_user))
}
