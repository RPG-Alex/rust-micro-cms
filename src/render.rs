use axum::{
    response::{Html, Json},
    extract::Extension,
};
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::Value;
use serde::Deserialize;
use std::convert::Infallible;
use crate::db::{self, fetch_single_post, Post};

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


pub async fn render_single_post_html(
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
pub async fn render_all_posts_html(db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Html<String> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match db::fetch_all_posts(&conn) {
        Ok(posts) => {
            let mut html_output = String::new();
            for post in posts.posts {
                // Each post title is a clickable link to the post's page
                html_output.push_str(&format!(
                    "<div><h2><a href=\"/post/{}\">{}</a></h2><p><i>{}</i></p><p>{}</p></div>",
                    post.id, post.title, post.author, post.body
                ));
            }

            Html(html_output)
        },
        Err(_) => Html("<div>Error Fetching All Posts</div>".to_string()),
    }
}

// Render form for adding a new post
pub async fn render_add_post_form() -> Html<String> {
    // HTML form for adding a new post
}

// Render form for editing a post
pub async fn render_edit_post_form(post_id: Path<usize>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Html<String> {
    // HTML form for editing an existing post
}

// Render confirmation for deleting a post
pub async fn render_delete_post_confirmation(post_id: Path<usize>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Html<String> {
    // HTML confirmation for deleting a post
}