mod api;
mod db;
mod render;

use axum::{
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

    // Get a connection from the pool
    let conn = pool.get().expect("Failed to get a connection from the pool");

    // Call the table creation methods
    db::create_author_table(&conn).expect("Failed to create author table");
    db::create_posts_table(&conn).expect("Failed to create posts table");
    db::add_author(&conn, "Sylvia").expect("failed to add");
    db::create_post(&conn, "Example Post", "12/22/2023", "lorem ipsum.", 1);
    // Set up the Axum application with routes
    let app = Router::new()
        .route("/", get(api::fetch_all_posts_as_json)) // Default route serves JSON version of all posts
        .route("/posts", get(render::render_all_posts)) // "/posts" route serves HTML version of all posts
        .route("/post/:id", get(render::render_single_post)) // "/post/:id" route serves individual post in HTML
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
