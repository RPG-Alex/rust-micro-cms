mod db;
use axum::{Json, Error};
use axum::{
    response::Html,
    routing::get,
    Router,
};

use rusqlite::Error as SqliteError;
//use rusqlite::types::Value;


use std::convert::Infallible;
use std::sync::{Arc, Mutex};
//Used for getting the socket address with Axum
use std::net::SocketAddr;
use std::path::Path;

mod render; 


#[tokio::main]
async fn main() {
    // Set path to database and make connection
    let db_path = Path::new("posts.db");
    let db_conn = db::establish_connection(&db_path);

    // Used for Testing. 
    
    
    
    let db_create_author = db::create_author_table(&db_conn.unwrap());

    let db_conn = db::establish_connection(&db_path);
    let db_add_author = db::add_author(&db_conn.unwrap(), "Mike Striker");

    let db_conn = db::establish_connection(&db_path);
    let db_create = db::create_posts_table(&db_conn.unwrap());
    let db_conn = db::establish_connection(&db_path);
    let db_post_insert = db::insert_post(&db_conn.unwrap(), "New Post Title", "2023-10-20", "This is the post body", 1).expect("Failed to insert post");
    //
    let db_conn = db::establish_connection(&db_path);

    // Fetch all posts and serialize to JSON
    let all_posts = Arc::new(Mutex::new(db::fetch_all_posts(&db_conn.unwrap())));

    let all_posts_as_json = fetch_all_posts_as_json(all_posts.clone()).await;
    // Currently outputting all posts to root path. Need to implement other paths
    let app = Router::new()
        .route("/", get(move || fetch_all_posts_as_json(all_posts)))
        .route("/posts", get( move || render::posts(all_posts_as_json)));
    // server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//API Endpoint for all posts
async fn fetch_all_posts_as_json(all_posts_from_db: Arc<Mutex<Result<db::Posts, SqliteError>>>) -> Result<Json<String>, Infallible> {
    match &*all_posts_from_db.lock().unwrap() {
        Ok(posts) => {
            let posts_json = serde_json::to_string(&posts.posts).unwrap();
            Ok(Json(posts_json))
        },
        Err(_) => Ok(Json("Error Fetching All Posts".to_string()),)
    }
}
