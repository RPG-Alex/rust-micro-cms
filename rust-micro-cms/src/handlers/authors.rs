use crate::database::authors::{fetch_author_by_id, insert_new_author};
use crate::models::authors::{Author, NewAuthor};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Response,
};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

// Handler to add a new author
pub async fn add_author(
    Json(new_author): Json<NewAuthor>,
    Extension(pool): Extension<Pool<SqliteConnectionManager>>,
) -> Result<Json<Author>, Response> {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    match insert_new_author(&conn, &new_author).await {
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
    Extension(pool): Extension<Pool<SqliteConnectionManager>>,
) -> Result<Json<Author>, Response> {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    match fetch_author_by_id(&conn, author_id).await {
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
