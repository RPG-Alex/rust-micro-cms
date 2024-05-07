use crate::database::posts::{
    delete_post, fetch_all_posts, fetch_post_by_id, insert_new_post, update_post,
};
use crate::models::posts::{NewPost, Post, Posts, UpdatePost};
use crate::state::AppState;
use axum::{
    debug_handler,
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Response,
};

// Handler to fetch all posts
#[debug_handler]
pub async fn get_all_posts(
    Extension(state): Extension<AppState>,
) -> Result<Json<Posts>, Response> {
    let conn = state.pool.get().expect("Failed to get DB connection from pool");
    match fetch_all_posts(&conn).await {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to retrieve posts".into())
            .unwrap()),
    }
}

pub async fn add_post(
    Json(new_post): Json<NewPost>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Post>, Response> {
    let conn = state.pool.get().expect("Failed to get DB connection from pool");
    match insert_new_post(&conn, &new_post).await {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to add post".into())
            .unwrap()),
    }
}
