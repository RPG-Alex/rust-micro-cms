mod db;
mod render;

use axum::{
    Json,
    routing::get,
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
        .route("/posts", get(render::fetch_all_posts_as_json))
        .route("/post/:id", get(render::post))
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


