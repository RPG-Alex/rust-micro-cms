use crate::database::authors::{fetch_author_by_id, insert_new_author};
use crate::models::authors::{Author, NewAuthor};
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Response,
};

// Handler to add a new author
pub async fn add_author(
    Json(new_author): Json<NewAuthor>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Author>, Response> {
    match insert_new_author(&pool, &new_author).await {
        Ok(author) => Ok(Json(author)),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to add author".into())
            .unwrap()),
    }
}

// Handler to fetch an author by ID
pub async fn get_author(
    Path(author_id): Path<i32>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Author>, Response> {
    match fetch_author_by_id(&pool, author_id).await {
        Ok(Some(author)) => Ok(Json(author)),
        Ok(None) => Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Author not found".into())
            .unwrap()),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to retrieve author".into())
            .unwrap()),
    }
}
