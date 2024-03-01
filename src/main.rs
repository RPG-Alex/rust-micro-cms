use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use dotenv::dotenv;
use std::env;

mod db;
mod controllers;
mod errors;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_path = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_connection = db::DBConnection::new(&db_path).await.expect("Failed to create db connection");

    db_connection.create_author_table().await.expect("Failed to create author table");

    let test_author = models::Author::new("John".to_string(), "Doe".to_string());
    let inserted_author = db_connection.insert_new_author(&test_author).await.expect("Failed to insert new author");
    println!("Inserted Author: {:?}", inserted_author);

    let author_service = Arc::new(services::AuthorService {
        db: Arc::new(db_connection), 
    });

    let app = Router::new()
    .merge(controllers::authors::app_routes(author_service));

    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Failed to bind");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
