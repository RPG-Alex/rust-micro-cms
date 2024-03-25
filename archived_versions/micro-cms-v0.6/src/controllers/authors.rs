use axum::{
    extract::Extension, Json, http::StatusCode, Router, routing::{get, post},
};
use crate::services::AuthorService;
use crate::models::{Author, Authors};
use anyhow::{Result, Error}; 

async fn create_author_handler(Extension(author_service): Extension<AuthorService>, Json(payload): Json<Author>) -> Result<Json<Author>, StatusCode> {
    author_service.create_author(&payload).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_all_authors_handler(Extension(author_service): Extension<AuthorService>) -> Result<Json<Authors>, StatusCode> {
    author_service.get_all_authors().await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn author_routes(author_service: AuthorService) -> Router {
    Router::new()
        .route("/authors", post(create_author_handler).get(get_all_authors_handler))
        .layer(axum::extract::Extension(author_service))
}
