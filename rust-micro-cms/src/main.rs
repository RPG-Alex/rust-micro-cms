use axum::{
    extract::Extension,
    http::{HeaderName, HeaderValue, Method},
};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod database;
mod errors;
mod handlers;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let state = state::AppState::new(&db_path);

    if let Err(e) = database::posts::create_posts_table(&state.pool).await {
        eprintln!("Failed to create posts table: {}", e);
        return;
    }

    if let Err(e) = database::styling::create_style_db(&state.pool).await {
        eprintln!("Failed to create styles table: {}", e);
        return;
    }

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://127.0.0.1:8080"))
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![HeaderName::from_static("content-type")]);


    let app = routes::app_routes()
        .await
        .layer(Extension(state))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
