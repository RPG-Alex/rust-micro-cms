use axum::{
    Form,
    Json,
    extract::Extension, Error,
    response::IntoResponse,
};
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::convert::Infallible;
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
struct NewPost {
    title: String,
    author_id: usize,
    body: String,
}

// Add a new post
pub async fn add_post(form: Form<NewPost>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Result<Json<serde_json::Value>, Error> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool");

    // Extracting data from the form
    let new_post = form.0;
    let date = Utc::now();
    // Inserting the new post into the database
    //      This needs to be corrected. 
    match db::create_post(&conn, &new_post.title, &date.to_string(), &new_post.body, new_post.author_id) {
        Ok(_) => Ok(Json(json!({ "status": "success", "message": "Post added successfully" }))),
        Err(e) => Ok(Json(json!({ "status": "error", "message": e.to_string() })))
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

// // Delete a post
// pub async fn delete_post(post_id: usize, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> StatusCode {
//     // Delete a post from the database
// }

// // Fetch all posts as JSON (moved from render.rs)
// pub async fn fetch_all_posts_as_json(db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Json<Vec<Post>> {
//     // Fetch all posts from the database and return as JSON
// }