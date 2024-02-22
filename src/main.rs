use anyhow::Result;
use axum::{
    routing::{delete, get, post}, Router
};
use chrono::Utc;
use db::posts;
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;

mod db;
mod errors;
mod models;
mod services;
mod utils;
use crate::models::Author;
use crate::models::Post;

#[tokio::main]
async fn main() {
    // 
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_connection = db::DBConnection::new(&database_url).await.unwrap();
    // instantiate the db tables if needed at runtime
    db_connection.create_posts_table().await.expect("Failed to create post table");
    db_connection.create_author_table().await.expect("Failed to create author table");

    // everything below is for hot testing (quick n dirty)
    let new_author = Author {
        id: None,
        name: String::from("John Doe"),
        deleted: None
    };

    db_connection.insert_new_author(&new_author).await.expect("Failed to insert author");

    let new_post = Post {
        id: None,
        title: "Example Title".to_string(),
        date: Utc::now().naive_utc().to_string(),
        body: "This is an example post body.".to_string(),
        author_id: 1,
    };

    let post = Post {
        id: None, 
        title: new_post.title,
        date: new_post.date,
        body: new_post.body,
        author_id: new_post.author_id,
    };

        match db_connection.insert_new_post(&post).await {
        Ok(inserted_post) => println!("Inserted post with ID: {}", inserted_post.id.unwrap()),
        Err(e) => eprintln!("Error inserting post: {}", e),
    }

    let posts = db_connection.fetch_all_posts().await;
    match posts {
        Ok(posts) => {
            for post in posts.posts {
                println!("ID: {}, Title: {}, Date: {}, Body: {}, Author ID: {}", 
                    post.id.unwrap(), 
                    post.title, 
                    post.date, 
                    post.body, 
                    post.author_id);
            }
        }
        Err(e) => eprintln!("We done messed up! {}", e),
    }
}
