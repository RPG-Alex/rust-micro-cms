use axum::{
    extract::{Extension, Path},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use crate::state::AppState;
use crate::models::{NewAuthor, UpdateAuthor};
use crate::database::authors;
use serde_json::json;


pub async fn create_author_handler(
    Extension(state): Extension<AppState>,
    Json(new_author): Json<NewAuthor>
) -> impl IntoResponse {
    let conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    };

    match authors::insert_new_author(&conn, &new_author).await {
        Ok(author) => {
            match serde_json::to_value(author) {
                Ok(json_value) => (StatusCode::CREATED, Json(json_value)),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})))
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

pub async fn get_all_authors_handler(
    Extension(state): Extension<AppState>
) -> impl IntoResponse {
    let conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    };

    match authors::fetch_all_authors(&conn).await {
        Ok(authors) => {
            match serde_json::to_value(authors) {
                Ok(json_value) => (StatusCode::OK, Json(json_value)),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})))
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

pub async fn get_author_by_id_handler(
    Extension(state): Extension<AppState>,
    Path(author_id): Path<i32>
) -> impl IntoResponse {
    let conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    };

    match authors::fetch_author_by_id(&conn, author_id).await {
        Ok(Some(author)) => {
            match serde_json::to_value(author) {
                Ok(json_value) => (StatusCode::OK, Json(json_value)),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})))
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Author not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

pub async fn update_author_handler(
    Extension(state): Extension<AppState>,
    Json(update_author): Json<UpdateAuthor>
) -> impl IntoResponse {
    let conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    };

    match authors::update_author(&conn, update_author).await {
        Ok(author) => {
            match serde_json::to_value(author) {
                Ok(json_value) => (StatusCode::OK, Json(json_value)),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Serialization failed: ".to_owned() + &e.to_string()})))
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}
