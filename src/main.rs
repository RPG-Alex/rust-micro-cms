mod api;
mod db;
mod render;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

#[tokio::main]
async fn main() {
    // Set path to database and establish a connection pool
    let db_path = Path::new("posts.db");

    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).expect("Failed to create pool.");

    // Set up the Axum application with routes
    let app = Router::new()
        .route("/", get(api::fetch_all_posts_as_json)) // Default route serves JSON version of all posts
        .route("/posts", get(render::render_all_posts)) // "/posts" route serves HTML version of all posts
        .route("/posts", post(api::add_post)) // add new posts
        .route("/post/:id", get(render::render_single_post)) // "/post/:id" route serves individual post in HTML
        .route("/post/new", get(render::render_add_post_form)) // create a new post
        .layer(axum::extract::Extension(Arc::new(pool)));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Run the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
