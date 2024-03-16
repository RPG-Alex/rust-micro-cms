use axum::{
    extract::Extension, Json, http::StatusCode, Router, routing::{get, post},
};

use handlebars::{DirectorySourceOptions, Handlebars};

use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener; 
use chrono::{NaiveDate, Utc};
mod db;
mod controllers;
mod errors;
mod models;
mod services;
mod utils;
mod ssr;
use crate::controllers::post_routes;
use crate::controllers::author_routes;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_path = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_connection = db::DBConnection::new(&db_path).await.expect("Failed to create db connection");

    db_connection.create_author_table().await.expect("Failed to create author table");

    let test_author = models::Author::new("John".to_string(), "Doe".to_string());
    let inserted_author = db_connection.insert_new_author(&test_author).await.expect("Failed to insert new author");
    println!("Inserted Author: {:?}", inserted_author);

    let test_post = models::Post::new("test post".to_string(), Utc::now().naive_local().date().to_string(), "This should be post data".to_string(), 1);
    let inserted_post = db_connection.insert_new_post(&test_post).await.expect("failed to insert test post");
    println!("Inserted post: {:?}", inserted_post);

    let author_service = services::AuthorService {
        db: db::DBConnection::new(&db_path).await.expect("Failed to create db connection"), 
    };

    let post_service = services::PostService {
        db: db::DBConnection::new(&db_path).await.expect("Failed to create db connection"),
    };

    let app = Router::new()

        .merge(author_routes(author_service).await)
        .merge(post_routes(post_service).await);

    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Failed to bind");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
