use axum::{
    extract::{Extension, Path}, Json, http::StatusCode, Router, routing::{get, post},
};
use crate::services::PostService;
use crate::models::{Post, Posts};
use anyhow::{Result, Error}; 

async fn create_post_handler(Extension(post_service): Extension<PostService>, Json(payload): Json<Post>) -> Result<Json<Post>, StatusCode> {
    post_service.create_post(&payload).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_all_posts_handler(Extension(post_service): Extension<PostService>) -> Result<Json<Posts>, StatusCode> {
    post_service.get_all_posts().await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_post_by_id_handler(
    Extension(post_service): Extension<PostService>, 
    Path(post_id): Path<i32>
) -> Result<Json<Post>, StatusCode> {
    post_service.get_post_by_id(post_id).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        
}

async fn put_post_update(Extension(post_service): Extension<PostService>,Json(payload): Json<Post>) -> Result<Json<Post>, StatusCode> {
    post_service.change_post(&payload).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn post_routes(post_service: PostService) -> Router {
    Router::new()
        .route("/posts", post(create_post_handler).get(get_all_posts_handler))
        .route("/posts/:id", post(put_post_update).get(get_post_by_id_handler))
        .layer(axum::extract::Extension(post_service))
}