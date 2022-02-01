use axum::{
    routing::{get, post},
    Router,
};

pub mod user;
pub fn app() -> Router {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users/login", post(user::login_user))
        .route("/user", get(user::get_current_user).put(user::update_user))
}
