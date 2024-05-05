use crate::database::posts::{
    delete_post, fetch_all_posts, fetch_post_by_id, insert_new_post, update_post,
};
use crate::models::posts::{NewPost, Post, Posts, UpdatePost};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Response,
};
use sqlx::SqlitePool;

// Handler to fetch all posts
pub async fn get_all_posts(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Posts>, Response> {
    match fetch_all_posts(&pool).await {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to retrieve posts".into())
            .unwrap()),
    }
}

// Handler to add a new post
pub async fn add_post(
    Json(new_post): Json<NewPost>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Post>, Response> {
    match insert_new_post(&pool, &new_post).await {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to add post".into())
            .unwrap()),
    }
}
