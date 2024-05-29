use crate::database::styling;
use crate::models::styling::NewStyle;
use crate::state::AppState;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn create_style_handler(
    Extension(state): Extension<AppState>,
    Json(new_style): Json<NewStyle>,
) -> impl IntoResponse {
    match styling::insert_style(&state.pool, new_style).await {
        Ok(style) => match serde_json::to_value(style) {
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

pub async fn fetch_styles_handler(Extension(state): Extension<AppState>) -> impl IntoResponse {
    match styling::fetch_all_styles(&state.pool).await {
        Ok(styles) => match serde_json::to_value(styles) {
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
