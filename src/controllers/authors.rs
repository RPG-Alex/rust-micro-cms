use axum::{
    Json, http::StatusCode, Router, routing::{get, post},
};
use crate::services::AuthorService;
use crate::models::Author;
use std::sync::Arc;

async fn create_author_handler(author_service: Arc<AuthorService>, Json(payload): Json<Author>) -> Result<Json<Author>, StatusCode> {
    author_service.create_author(&payload).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_all_authors_handler(author_service: Arc<AuthorService>) -> Result<Json<Authors>, StatusCode> {
    author_service.get_all_authors().await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn app_routes(author_service: Arc<AuthorService>) -> Router {
    Router::new()
        .route("/authors", post(create_author_handler).get(get_all_authors_handler))
        .layer(axum::extract::Extension(author_service))
}
