use axum::{
    routing::{delete, get, post}, Router
};
use chrono::Utc;
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use anyhow::Result;
mod db;
mod errors;
mod models;
mod services;
mod utils;
use crate::models::Author;
use crate::models::Post;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_connection = db::DBConnection::new(&database_url).await.unwrap();
    db_connection.create_posts_table().await.expect("Failed to create post table");
    db_connection.create_author_table().await.expect("Failed to create author table");

    let new_author = Author {
        id: None,
        name: String::from("John Doe"),
    };

    db_connection.insert_new_author(&new_author).await.expect("Failed to insert author");

    let new_post = Post {
        id: None,
        title: "Example Title".to_string(),
        date: Utc::now().naive_utc(),
        body: "This is an example post body.".to_string(),
        author_id: 1, // Assuming you have an author with ID 1
    };

    // Convert NewPost to Post (if necessary)
    let post = Post {
        id: None, // ID will be set by the database
        title: new_post.title,
        date: new_post.date,
        body: new_post.body,
        author_id: new_post.author_id,
    };

    // Insert the post into the database
        match db_connection.insert_new_post(&post).await {
        Ok(inserted_post) => println!("Inserted post with ID: {}", inserted_post.id.unwrap()),
        Err(e) => eprintln!("Error inserting post: {}", e),
    }
}
