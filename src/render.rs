use axum::{
    response::Html,
    extract::{Path, Extension},
};

use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use crate::db::{self, fetch_single_post};

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


pub async fn render_single_post(
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
pub async fn render_all_posts(db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Html<String> {
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
    Html(
        "<form action='/add_post' method='post'>
        <input type='text' name='title' id='title'>
        <input type='text' name='body' id='body'>
        <input type='submit' value='Add Post'>
        </form>
    ".to_string()
    )
}

// Render form for editing a post
pub async fn render_edit_post_form(post_id: Path<usize>, db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>) -> Html<String> {
    let post_id = *post_id;

    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match fetch_single_post(&conn, post_id) {
        Ok(Some(post)) => {
            let post_html = format!(
                "<form action='/update_post' method='post'>
                <input type='text' name='title' id='title' value='{}'>
                <input type='hidden' name='id' value='{}'>
                <input type='text' name='body' id='body' value='{}'>
                <input type='submit' value='Add Post'>
                </form>
            ",
                post.title, post.id, post.body
            );
            Html(post_html)
        },
        Ok(None) => Html(format!("<div>No post found with ID {}</div>", post_id)),
        Err(_) => Html("<div>Error fetching post</div>".to_string()),
    }
}

// Render confirmation for deleting a post
pub async fn render_delete_post_confirmation(
    post_id: Path<usize>, 
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>
) -> Html<String> {
    let post_id = *post_id;
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match fetch_single_post(&conn, post_id) {
        Ok(Some(post)) => {
            let confirmation_html = format!(
                "<div>
                    <p>Are you sure you want to delete the post: '{}'?</p>
                    <form action='/delete_post' method='post'>
                        <input type='hidden' name='id' value='{}'>
                        <input type='submit' value='Confirm Delete'>
                        <a href='/cancel_delete'>Cancel</a>
                    </form>
                </div>",
                post.title, post.id
            );
            Html(confirmation_html)
        },
        Ok(None) => Html(format!("<div>No post found with ID {}</div>", post_id)),
        Err(_) => Html("<div>Error fetching post for deletion</div>".to_string()),
    }
}