use axum::{
    http::StatusCode,
    Json,
    extract::Extension,
    response::IntoResponse,
};
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::db::{self};



// Structure used for submitting new posts
#[derive(Deserialize)]
pub struct NewPost {
    title: String,
    author_id: usize,
    body: String,
}

// Structure for updating a post
#[derive(Deserialize)]
pub struct UpdatePost {
    title: String,
    post_id: usize,
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

// Delete a post
pub async fn delete_post(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>,
    Json(post_id): Json<usize>,
) -> impl IntoResponse {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool");

    
    match db::delete_post(&conn, &post_id) {
        Ok(_) => (
            StatusCode::OK,
            Json(PostResponse{
                status: "success".to_string(),
                message: "Post deleted successfully".to_string(),
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

// Update an existing post
pub async fn update_post(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>,
    Json(update_post): Json<UpdatePost>,  
) -> impl IntoResponse  {
        let pool = db_pool.0;
        let conn = pool.get().expect("Failed to get a connection from the pool");

        //need to modify later, to make updating date optional
        let date = Utc::now().naive_local().date();
        match db::update_post(
            &conn, 
            update_post.post_id, 
            &update_post.title,
            update_post.author_id, 
            &date.to_string(), 
            &update_post.body) {
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


#[derive(Deserialize)]
pub struct NewAuthor {
    author_name: String,
}
#[derive(Deserialize)]
pub struct UpdateAuthor {
    pub author_id: usize,
    pub author: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    status: String,
    message: String, 
}

pub async fn add_author(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>,
    Json(new_name) : Json<NewAuthor>,
) -> impl IntoResponse {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from pool");
    let author_name = &new_name.author_name;
    match db::add_author(&conn, author_name) {
        Ok(_) => (
            StatusCode::OK,
            Json(AuthResponse {
                status: "success".to_string(),
                message: "Author added successfully".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                status: "error".to_string(),
                message: e.to_string(),
            })
        ),
    }
}

pub async fn fetch_author_info_as_json(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>,
    Json(author_id) : Json<usize>,
) -> impl IntoResponse{
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match db::fetch_author_info(&conn, author_id) {
        Ok(author) => {
            let author_json = serde_json::to_string(&author).expect("Failed to serialize Author Info.");
            Json(author_json)
        },
        Err(_) => Json("Error Fetching Author Info".to_string())
    }
}

pub async fn update_author(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>,
    Json(update_name) : Json<UpdateAuthor>,
) -> impl IntoResponse {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from pool");
    match db::update_author(&conn, update_name.author_id, &update_name.author) {
        Ok(_) => (
            StatusCode::OK,
            Json(AuthResponse {
                status: "success".to_string(),
                message: "Author updated successfully".to_string(),
            }),
        ),
        Err(rusqlite::Error::QueryReturnedNoRows) => (
            StatusCode::NOT_FOUND,
            Json(AuthResponse {
                status: "error".to_string(),
                message: "Author not found".to_string(),
            })
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                status: "error".to_string(),
                message: e.to_string(),
            })
        ),
    }
}

pub async fn fetch_all_authors_as_json(
    db_pool: Extension<Arc<Pool<SqliteConnectionManager>>>
) -> Json<String> {
    let pool = db_pool.0;
    let conn = pool.get().expect("Failed to get a connection from the pool.");

    match db::fetch_all_authors(&conn) {
        Ok(authors) => {
            let authors_json = serde_json::to_string(&authors.authors).expect("Failed to serialize Authors.");
            Json(authors_json)
        },
        Err(_) => Json("Error Fetching All Authors".to_string())
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
    async fn test_add_post_fail() {
        let db_pool = create_test_db_pool();
        let invalid_post = NewPost {
            title: "Invalid Post".to_string(),
            author_id: 999,
            body: "Content".to_string(),
        };
        let response = add_post(db_pool, Json(invalid_post)).await.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_delete_post() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let post_id_to_delete = 1;
        let response = delete_post(db_pool, Json(post_id_to_delete)).await.into_response();
        let (parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("Failed to read response body");
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");

        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(response_value["status"], "success");
        assert_eq!(response_value["message"], "Post deleted successfully");
    }

    #[tokio::test]
    async fn test_delete_post_fail() {
        let db_pool = create_test_db_pool();
        let non_existent_post_id = Json(999); 
        let response = delete_post(db_pool, non_existent_post_id).await.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_post() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        let update_post_data = UpdatePost {
            title: "Updated Post".to_string(),
            post_id: 1,
            author_id: 1,
            body: "Updated Post Content".to_string(),
        };
        let response = update_post(db_pool, Json(update_post_data)).await.into_response();
        let (parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("Failed to read response body");
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");
    
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(response_value["status"], "success");
        assert_eq!(response_value["message"], "Post updated successfully");
    }
    
    #[tokio::test]
    async fn test_update_post_fail() {
        let db_pool = create_test_db_pool();
        let invalid_update = Json(UpdatePost {
            title: "Updated Title".to_string(),
            post_id: 999,
            author_id: 1,
            body: "Updated Content".to_string(),
        });
        let response = update_post(db_pool, invalid_update).await.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_add_author() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);

        let new_author = NewAuthor {
            author_name: "John".to_string(),
        };
        let response = add_author(db_pool, Json(new_author)).await.into_response();
        let (parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("Failed to read response body");
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");

        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(response_value["status"], "success");
        assert_eq!(response_value["message"], "Author added successfully");
    }

    #[tokio::test]
    async fn test_add_author_fail() {
        let db_pool = create_test_db_pool();
        let invalid_author = Json(NewAuthor { author_name: "".to_string() });
        let response = add_author(db_pool, invalid_author).await.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_author() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().unwrap();
        setup_test_database(&conn);

        let update_author_data = UpdateAuthor {
            author_id: 1,
            author: "Jane Doe".to_string(),
        };
        let response = update_author(db_pool, Json(update_author_data)).await.into_response();
        let (_, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.unwrap();
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(response_value["status"], "success");
        assert_eq!(response_value["message"], "Author updated successfully");
    }

    #[tokio::test]
    async fn test_update_author_fail_auth_not_found(){
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);

        let invalid_update_author_data = Json(UpdateAuthor {
            author_id: 999,
            author: "Non Existent Author".to_string(),
        });
        let response = update_author(db_pool, invalid_update_author_data).await.into_response();
        let (parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("Failed to read response body");
        let response_value: serde_json::Value = serde_json::from_slice(&body_bytes).expect("Failed to parse JSON");

        assert_eq!(parts.status, StatusCode::NOT_FOUND);

        assert_eq!(response_value["status"], "error");
    }

    #[ignore]
    #[tokio::test]
    async fn test_update_author_fail_int_err(){
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().expect("Failed to get a connection from the pool");
        setup_test_database(&conn);
    
        // Use an author_id that does not exist to simulate a failure condition
        let nonexistent_author_id = 9999; // Assuming this ID does not exist in your test database
        let update_author_data = UpdateAuthor {
            author_id: nonexistent_author_id,
            author: "Jane Doe".to_string(),
        };
    
        let response = update_author(db_pool, Json(update_author_data)).await;
        let (parts, _) = response.into_response().into_parts();
    
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_fetch_all_authors_as_json() {
        let db_pool = create_test_db_pool();
        let conn = db_pool.0.get().unwrap();
        setup_test_database(&conn);


        let response = fetch_all_authors_as_json(db_pool).await;
        let response_str = &response.0;
        let response_value: serde_json::Value = serde_json::from_str(response_str).unwrap();

        assert!(response_value.is_array());
        let authors_array = response_value.as_array().unwrap();
        assert!(authors_array.iter().any(|author| author["author"] == "John Doe"));
    }

    #[tokio::test]
    async fn test_fetch_all_authors_fail() {
        let db_pool = create_test_db_pool();
        let response = fetch_all_authors_as_json(db_pool).await;
        assert_eq!(response.to_string(), "Error Fetching All Authors".to_string());
    }

}
