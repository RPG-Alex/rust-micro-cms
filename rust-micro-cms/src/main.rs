use axum::extract::Extension;
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;

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

    
    let app = routes::app_routes().await
        .layer(Extension(state));

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
