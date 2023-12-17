use axum::{
    response::{Html, Json},
    extract::Extension,
};
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::Value;
use serde::{Deserialize};
use std::convert::Infallible;
use crate::db::{self, fetch_single_post, Post};
//use rusqlite::Error as SqliteError;
// Temporary structure to deserialize each post from JSON
#[derive(Deserialize)]
pub struct TempPost {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub body: String,
    pub author_id: usize,
    pub author: String,
}

// Deserialize the json and structure as html and return it
pub async fn posts(posts: Result<Json<String>, Infallible>) -> Html<String> {
    match posts {
        Ok(Json(json_string)) => {
            // Parse the JSON string to a serde_json::Value
            let posts_data: Value = serde_json::from_str(&json_string).unwrap();

            // Check if posts_data is an array and iterate over it
            if posts_data.is_array() {
                let posts_array = posts_data.as_array().unwrap();
                let mut html_output = String::new();

                for post_value in posts_array {
                    // Deserialize each post
                    let post: TempPost = serde_json::from_value(post_value.clone()).unwrap();

                    // Create HTML string for each post
                    html_output.push_str(&format!(
                        "<div><h2>{}</h2><p><i>{}</i></p><p>{}</p></div>",
                        post.title, post.author, post.body
                    ));
                }

                Html(html_output)
            } else {
                Html("<div>Error: Posts data is not an array.</div>".to_string())
            }
        }
        Err(_) => Html("<div>Unable to parse JSON</div>".to_string()),
    }
}

pub async fn post(
    post_id: axum::extract::Path<usize>, 
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>
) -> Html<String> {
    let post_id = *post_id;

    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match fetch_single_post(&conn, post_id) {
        Ok(Some(post)) => {
            let post_html = format!(
                "<div><h2>{}</h2><p><i>{}</i></p><p>{}</p></div>",
                post.title, post.author, post.body
            );
            Html(post_html)
        },
        Ok(None) => Html(format!("<div>No post found with ID {}</div>", post_id)),
        Err(_) => Html("<div>Error fetching post</div>".to_string()),
    }
}

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