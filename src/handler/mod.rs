use axum::{Router, routing::post};

pub mod user;
pub fn app()->Router{
    Router::new()
        .route("/users", post(user::create_user))
}
