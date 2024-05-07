use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use std::env;
use tokio::net::TcpListener;

mod database;
mod handlers;
mod models;
mod state;
mod errors;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).expect("Failed to create pool");

    let state = state::AppState::new(pool);

    let app = Router::new()
        //.route("/posts", get(handlers::posts::get_all_posts))
        //.route("/authors/:id", get(handlers::authors::get_author))
        .layer(Extension(state));  // Passing the AppState

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
