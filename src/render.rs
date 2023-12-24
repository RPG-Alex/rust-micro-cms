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
        "<style>
            form {
                display: flex;
                flex-direction: column;
                width: 400px;
                margin: 20px;
            }
            label {
                margin-top: 10px;
            }
            input[type='text'], textarea {
                padding: 8px;
                margin-top: 5px;
                border-radius: 4px;
                border: 1px solid #ddd;
                font-size: 16px;
            }
            input[type='submit'] {
                margin-top: 20px;
                padding: 10px;
                border: none;
                background-color: #4CAF50;
                color: white;
                border-radius: 4px;
                cursor: pointer;
                font-size: 16px;
            }
            input[type='submit']:hover {
                background-color: #45a049;
            }
        </style>
        <form id='addPostForm'>
            <label for='title'>Title</label>
            <input type='text' name='title' id='title' placeholder='Enter post title' required>

            <label for='author'>Author</label>
            <input type='text' name='author' id='author' placeholder='Enter author name' required>

            <label for='body'>Body</label>
            <textarea name='body' id='body' placeholder='Enter post content' rows='6' required></textarea>

            <input type='submit' value='Add Post'>
        </form>
        <script>
            document.getElementById('addPostForm').addEventListener('submit', function(e) {
                e.preventDefault();

                var title = document.getElementById('title').value;
                var author = document.getElementById('author').value;
                var body = document.getElementById('body').value;

                fetch('/add_post', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ title, author, body })
                })
                .then(response => response.status)
                .then(status => console.log('Submitted with status:', status))
                .catch(error => console.error('Error:', error));
            });
        </script>
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