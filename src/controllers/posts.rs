use axum::{
    extract::Extension, Json, http::StatusCode, Router, routing::{get, post},
};
use crate::services::PostService;
use crate::models::{Post, Posts};
use std::sync::Arc;
use anyhow::{Result, Error}; 

async fn create_post_handler(Extension(post_service): Extension<Arc<PostService>>,  Json(payload): Json<Post>) -> Result<Json<Post>, StatusCode> {
    post_service.create_post(&payload).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}