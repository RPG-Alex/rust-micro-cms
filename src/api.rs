use axum::{
    http::StatusCode,
    Json,
    extract::Extension, Error,
    response::IntoResponse,
    
};
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::sync::Arc;
use crate::db::{self};

//API Endpoint for all posts
pub async fn fetch_all_posts_as_json(db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Json<String> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match db::fetch_all_posts(&conn) {
        Ok(posts) => {
            let posts_json = serde_json::to_string(&posts.posts).expect("Failed to serialize posts.");
            Json(posts_json)
        },
        Err(_) => Json("Error Fetching All Posts".to_string())
    }
}


// Structure used for submitting new posts
#[derive(Deserialize)]
pub struct NewPost {
    title: String,
    author_id: usize,
    body: String,
}

// Response type for successful post creation (thank you documentation!)
#[derive(Serialize)]
pub struct PostResponse {
    status: String,
    message: String,
}

// Add a new post
pub async fn add_post(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>,
    Json(new_post): Json<NewPost>, 
) -> impl IntoResponse {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool");

    let date = Utc::now().naive_local().date();
    match db::create_post(&conn, &new_post.title, &date.to_string(), &new_post.body, new_post.author_id) {
        Ok(_) => (
            StatusCode::OK,
            Json(PostResponse {
                status: "success".to_string(),
                message: "Post added successfully".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PostResponse {
                status: "error".to_string(),
                message: e.to_string(),
            }),
        ),
    }
}

// Delete a post
pub async fn delete_post(post_id: usize, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Result<Json<serde_json::Value>, Error> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool");

    
    match db::delete_post(&conn, &post_id) {
        Ok(_) => Ok(Json(json!({ "status": "success", "message": "Post deleted successfully" }))),
        Err(e) => Ok(Json(json!({ "status": "error", "message": e.to_string() })))
    }
}

// // Update an existing post
// pub async fn update_post(post_id: usize, form: Form<UpdatePost>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> StatusCode {
//     // Update an existing post in the database
// }


