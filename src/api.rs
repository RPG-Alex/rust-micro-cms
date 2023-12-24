use axum::{
    Form,
    response::Json,
    extract::Extension,
};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use std::convert::Infallible;
use crate::db::{self, fetch_single_post, Post};

//API Endpoint for all posts
pub async fn fetch_all_posts_as_json(db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Result<Json<String>, Infallible> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match db::fetch_all_posts(&conn) {
        Ok(posts) => {
            let posts_json = serde_json::to_string(&posts.posts).expect("Failed to serialize posts.");
            Ok(Json(posts_json))
        },
        Err(_) => Ok(Json("Error Fetching All Posts".to_string()))
    }
}


// Structure used for submitting new posts
#[derive(Deserialize)]
struct New_Post {
    title: String,
    author: String,
    body: String,
}

// Add a new post
pub async fn add_post(form: Form<New_Post>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> StatusCode {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool");
    match db::create_post(conn, , date, body, author_id)
}

// // Update an existing post
// pub async fn update_post(post_id: usize, form: Form<UpdatePost>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> StatusCode {
//     // Update an existing post in the database
// }

// // Delete a post
// pub async fn delete_post(post_id: usize, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> StatusCode {
//     // Delete a post from the database
// }

// // Fetch all posts as JSON (moved from render.rs)
// pub async fn fetch_all_posts_as_json(db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Json<Vec<Post>> {
//     // Fetch all posts from the database and return as JSON
// }