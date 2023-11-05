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

    // Database Testing logic
    let db_path = Path::new("posts.db");
    let db_conn = db::establish_connection(&db_path);
    let db_create = db::create_posts_table(&db_conn.unwrap());
    let db_conn = db::establish_connection(&db_path);
    let db_insert = db::insert_post(&db_conn.unwrap(), "New Post Title", "2023-10-20", "This is the post body")
    .expect("Failed to insert post");
    let db_conn = db::establish_connection(&db_path);
    let all_posts = db::fetch_all_posts(&db_conn.unwrap());
    print!("{:#?}",all_posts);
    //

    // server address
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

