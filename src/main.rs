use axum::{
    routing::{delete, get, post}, Router
};
use chrono::Utc;
use db::posts;
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use std::sync::Arc;
use std::net::SocketAddr;

mod db;
mod controllers;
mod errors;
mod models;
mod services;
mod utils;
use crate::controllers::authors::app_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Setup the database connection
    let db_path = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Asynchronously create the DB connection and handle the Result
    let db_connection = db::DBConnection::new(&db_path).await.expect("Failed to create db connection");

    // Create the service layer with the DB connection wrapped in an Arc
    let author_service = Arc::new(services::AuthorService {
        db: Arc::new(db_connection), // Ensure the connection is wrapped in an Arc correctly
    });

    // Setup the app's routes, passing the author service
    let app = app_routes(author_service);

    // Define the address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}