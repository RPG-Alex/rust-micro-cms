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

// Update an existing post
pub async fn update_post(
    post_id: usize, 
    Json(update_post): Json<NewPost>,  
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>
) -> impl IntoResponse  {
        let pool = db_pool.0;
        let conn = pool.get().expect("Failed to get a connection from the pool");

        //need to modify later, to make updating date optional
        let date = Utc::now().naive_local().date();
        match db::update_post(&conn, post_id, &update_post.title, &date.to_string(), &update_post.body) {
            Ok(_) => (
                StatusCode::OK,
                Json(PostResponse {
                    status: "success".to_string(),
                    message: "Post updated successfully".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Extension;
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


    // Test helper functions first to make sure they are testing our mock db properly
    #[test]
    fn test_create_test_db_pool() {
        let db_pool = create_test_db_pool();
        let conn_result = db_pool.0.get();
        assert!(conn_result.is_ok(), "Should be able to get a connection from the pool");
    }

    #[test]
    fn test_setup_test_database() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);

        // Verify author table exists and has data
        let author_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM author",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(author_count, 1, "Author table should have 1 row");

        // Verify posts table exists and has data
        let posts_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM posts",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(posts_count, 1, "Posts table should have 1 row");
    }

    #[tokio::test]
    async fn test_fetch_all_posts_as_json() {
        let db_pool = create_test_db_pool();
    
        // Set up the database with necessary data
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let response = fetch_all_posts_as_json(db_pool).await;
        let response_str = &response.0;
        println!("Response: {}", response_str); // Print the response for debugging
    
        let response_value: serde_json::Value = serde_json::from_str(response_str)
            .expect("Failed to parse JSON");
    
        // Assert that the response contains the expected data
        assert!(response_value.is_array());
        let posts_array = response_value.as_array().unwrap();
        assert_eq!(posts_array.len(), 1);
        assert_eq!(posts_array[0]["title"], "Test Post");
        assert_eq!(posts_array[0]["body"], "Test Content");
        assert_eq!(posts_array[0]["author"], "John Doe");
    }

    #[tokio::test]
    async fn test_add_post() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let new_post = NewPost {
            title: "New Post".to_string(),
            author_id: 1,
            body: "New Post Content".to_string(),
        };
        let response = add_post(db_pool, Json(new_post)).await.into_response();
        let (parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("Failed to read response body");
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(response_value["status"], "success");
        assert_eq!(response_value["message"], "Post added successfully");
    }
    
    #[tokio::test]
    async fn test_delete_post() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let post_id_to_delete = 1;
        let response = delete_post(post_id_to_delete, db_pool).await.expect("Failed to delete post");
        assert_eq!(response["status"], "success");
        assert_eq!(response["message"], "Post deleted successfully");
    }

    #[tokio::test]
    async fn test_update_post() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let update_post_data = NewPost {
            title: "Updated Post".to_string(),
            author_id: 1,
            body: "Updated Post Content".to_string(),
        };
        let post_id_to_update = 1;
        let response = update_post(post_id_to_update, Json(update_post_data), db_pool).await.into_response();
        let (parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("Failed to read response body");
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(response_value["status"], "success");
        assert_eq!(response_value["message"], "Post updated successfully");
    }
    
}
