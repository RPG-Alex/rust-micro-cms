use crate::database::posts;
use crate::models::{NewPost, UpdatePost};
use crate::state::AppState;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn create_post_handler(
    Extension(state): Extension<AppState>,
    Json(new_post): Json<NewPost>,
) -> impl IntoResponse {
    match posts::insert_new_post(&state.pool, &new_post).await {
        Ok(post) => match serde_json::to_value(post) {
            Ok(json_value) => (StatusCode::CREATED, Json(json_value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn get_all_posts_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    match posts::fetch_all_posts(&state.pool).await {
        Ok(posts) => match serde_json::to_value(posts) {
            Ok(json_value) => (StatusCode::OK, Json(json_value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn get_post_by_id_handler(
    Extension(state): Extension<AppState>,
    Path(post_id): Path<i32>,
) -> impl IntoResponse {
    match posts::fetch_post_by_id(&state.pool, post_id).await {
        Ok(Some(post)) => match serde_json::to_value(post) {
            Ok(json_value) => (StatusCode::OK, Json(json_value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})),
            ),
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post not found"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn update_post_handler(
    Extension(state): Extension<AppState>,
    Json(update_post): Json<UpdatePost>,
) -> impl IntoResponse {
    match posts::update_post(&state.pool, update_post).await {
        Ok(post) => match serde_json::to_value(post) {
            Ok(json_value) => (StatusCode::OK, Json(json_value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn delete_post_handler(
    Extension(state): Extension<AppState>,
    Path(post_id): Path<i32>,
) -> impl IntoResponse {
    match posts::delete_post(&state.pool, post_id).await {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({"message": "Post deleted successfully"})),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post not found"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
