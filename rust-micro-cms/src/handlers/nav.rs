use crate::database::nav;
use crate::models::nav::NewNavItem;
use crate::state::AppState;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn create_nav_item_handler(
    Extension(state): Extension<AppState>,
    Json(new_nav_item): Json<NewNavItem>,
) -> impl IntoResponse {
    match nav::insert_new_item(&state.pool, new_nav_item).await {
        Ok(item) => match serde_json::to_value(item) {
            Ok(json_value) => (StatusCode::CREATED, Json(json_value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error!": "Serialization Failed: ".to_owned() + &e.to_string()})),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn fetch_nav_items_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    match nav::fetch_all_nav_items(&state.pool).await {
        Ok(items) => match serde_json::to_value(items) {
            Ok(json_value) => (StatusCode::OK, Json(json_value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error!":"Serialization Failed".to_owned() + &e.to_string()})),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

