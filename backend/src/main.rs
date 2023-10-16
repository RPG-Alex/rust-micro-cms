/* TODO:
- CREATE REST API for frontend to access
    - Create database control structure
    - Add functions for reading from database first
    - Implement HTTP GET request

*/
use axum::{
    response::Html,
    routing::get,
    Router,
};
//Used for getting the socket address with Axum
use std::net::SocketAddr;

//Post Structure
struct Post {
    id: usize,
    title: String,
    date: String,
    body: String,
}

// Structure for vector of posts (such as fetching all from DB)
struct Posts {
    posts: Vec<Post>,
}


#[tokio::main]
async fn main() {
    //Currently taken from the Axum Example
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Axum Example handler
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

