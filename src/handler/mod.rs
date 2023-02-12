use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::DbPool;

pub mod article;
pub mod user;

pub fn app_router(db_pool: DbPool) -> Router {
    let state = Arc::new(AppState { db: db_pool });

    let article_router = Router::new().route(
        "/articles",
        post(article::create_article).get(article::list_most_recent_articles),
    );

    let user_router = Router::new()
        .route("/users", post(user::create_user))
        .route("/users/login", post(user::login_user))
        .route("/user", get(user::get_current_user).put(user::update_user));
    let router = user_router.merge(article_router);
    Router::new()
        .nest("/api", router)
        .with_state(state)
        .fallback(handler_404)
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DbPool,
}

async fn handler_404() -> impl axum::response::IntoResponse {
    (
        http::StatusCode::NOT_FOUND,
        [(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )],
        r#""errors":{"msg"="404 not found"}"#,
    )
}
