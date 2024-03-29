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
                    "<div><h2><a href=\"/post/{}\">{}</a></h2><p><i>date: {}</i></p><p><i>{}</i></p><p>{}</p></div>",
                    post.id, post.title, post.date, post.author, post.body
                ));
            }

            Html(html_output)
        },
        Err(_) => Html("<div>Error Fetching All Posts</div>".to_string()),
    }
}

// Render form for adding a new post
pub async fn render_add_post_form(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>
) -> Html<String> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");
    let authors_options = match db::fetch_all_authors(&conn) {
        Ok(authors) => {
            // Iterate over authors to create option elements
            authors.authors.iter().map(|author| {
                format!("<option value='{}'>{}</option>", author.author_id, author.author)
            }).collect::<Vec<_>>().join("\n")
        },
        Err(_) => {
            // Fallback option in case of an error
            "<option value=''>Failed to fetch authors</option>".to_string()
        }
    };

    let form_html = format!(
        "<style>
            form {{
                display: flex;
                flex-direction: column;
                width: 400px;
                margin: 20px;
            }}
            label {{
                margin-top: 10px;
            }}
            input[type='text'], textarea {{
                padding: 8px;
                margin-top: 5px;
                border-radius: 4px;
                border: 1px solid #ddd;
                font-size: 16px;
            }}
            input[type='submit'] {{
                margin-top: 20px;
                padding: 10px;
                border: none;
                background-color: #4CAF50;
                color: white;
                border-radius: 4px;
                cursor: pointer;
                font-size: 16px;
            }}
            input[type='submit']:hover {{
                background-color: #45a049;
            }}
        </style>
        <form id='addPostForm'>
            <label for='title'>Title</label>
            <input type='text' name='title' id='title' placeholder='Enter post title' required>

            <label for='author_id'>Author</label>
            <select name='author_id' id='author_id' required>
                {authors_options}
            </select>

            <label for='body'>Body</label>
            <textarea name='body' id='body' placeholder='Enter post content' rows='6' required></textarea>

            <input type='submit' value='Add Post'>
        </form>
        <script>
            document.getElementById('addPostForm').addEventListener('submit', function(e) {{
                e.preventDefault();

                var title = document.getElementById('title').value;
                var author_id = parseInt(document.getElementById('author_id').value, 10);
                var body = document.getElementById('body').value;

                fetch('/post/add_post', {{
                    method: 'POST',
                    headers: {{
                        'Content-Type': 'application/json',
                    }},
                    body: JSON.stringify({{ title, author_id, body }})
                }})
                .then(response => response.json())
                .then(data => {{
                    // Handle response data, e.g., redirect to a success page or display a message
                    console.log('Submission successful:', data);
                }})
                .catch(error => console.error('Error:', error));
            }});
        </script>");

    Html(form_html)
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

pub async fn render_success_view() -> Html<String> {
    Html("<div>Post submitted successfully!</div>".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::{Extension, Path};
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::Connection;
    use std::sync::Arc;

    // Helper function to create a test database pool
    fn create_test_db_pool() -> Extension<Arc<Pool<SqliteConnectionManager>>> {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).expect("Failed to create a test database pool");
        Extension(Arc::new(pool))
    }

    // Helper function to set up the database with necessary data
    fn setup_test_database(conn: &Connection) {
        conn.execute("CREATE TABLE IF NOT EXISTS author (id INTEGER PRIMARY KEY, author TEXT NOT NULL)", [])
            .expect("Failed to create author table");
        conn.execute("CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY, title TEXT NOT NULL, date TEXT NOT NULL, body TEXT NOT NULL, author_id INTEGER NOT NULL, FOREIGN KEY (author_id) REFERENCES author(id))", [])
            .expect("Failed to create posts table");
        conn.execute("INSERT INTO author (author) VALUES (?1)", ["John Doe"])
            .expect("Failed to insert author");
        conn.execute("INSERT INTO posts (title, date, body, author_id) VALUES (?1, ?2, ?3, ?4)", ["Test Post", "2022-01-01", "Test Content", &1.to_string()])
            .expect("Failed to insert post");
    }

    #[tokio::test]
    async fn test_render_single_post() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let post_id = 1;
        let response = render_single_post(Path(post_id), db_pool).await;
        assert!(response.0.contains("Test Post"));
        assert!(response.0.contains("Test Content"));
        assert!(response.0.contains("John Doe"));
    }
    

    #[tokio::test]
    async fn test_render_all_posts() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let response = render_all_posts(db_pool).await;
        assert!(response.0.contains("Test Post"));
        assert!(response.0.contains("Test Content"));
        assert!(response.0.contains("John Doe"));
    }
    

    #[tokio::test]
    async fn test_render_add_post_form() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
        let response = render_add_post_form(db_pool).await;
        assert!(response.0.contains("<form id='addPostForm'>"));
        assert!(response.0.contains("<input type='submit' value='Add Post'>"));
    }
    

    #[tokio::test]
    async fn test_render_edit_post_form() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let post_id = 1;
        let response = render_edit_post_form(Path(post_id), db_pool).await;
        assert!(response.0.contains("value='Test Post'"));
        assert!(response.0.contains("value='Test Content'"));
    }
    

    #[tokio::test]
    async fn test_render_delete_post_confirmation() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let post_id = 1;
        let response = render_delete_post_confirmation(Path(post_id), db_pool).await;
        assert!(response.0.contains("Are you sure you want to delete the post:"));
        assert!(response.0.contains("value='Confirm Delete'"));
    }
    
}
