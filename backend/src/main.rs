/* TODO:
- CREATE REST API for frontend to access
    - Create database control structure
    - Add functions for reading from database first
    - Implement HTTP GET request

*/
mod db;
use axum::{
    response::Html,
    routing::get,
    Router,
};
//Used for getting the socket address with Axum
use std::net::SocketAddr;
use std::path::Path;



#[tokio::main]
async fn main() {
    //Currently taken from the Axum Example
    let app = Router::new().route("/", get(handler));


    let db_path = Path::new(".");
    let db_conn = db::establish_connection(&db_path);
    let db_create = db::create_posts_table(db_conn);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Axum Example handler
async fn handler() -> Html<&'static str> {
    Html("<h1>Work in Progress</h1>")
}