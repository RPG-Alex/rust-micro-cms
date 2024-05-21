use crate::{database::styling, state};
use crate::models::styling::Style;
use crate::state::AppState;
use axum::response::IntoResponseParts;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn create_style_handler(
    Extension(state): Extension<AppState>,
    Json(new_style): Json<Style>,
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
        )
    }
}