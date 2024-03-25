mod api;
mod db;
mod render;

use axum::{
    routing::{delete, get, post}, Router
};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

#[tokio::main]
async fn main() {
    let db_path = Path::new("posts.db");
    
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).expect("Failed to create pool.");

    initialize_database_tables(&pool).await.expect("Failed to initialize database tables");


        let app = Router::new()
        // App root and APIs
        .route("/posts", get(api::fetch_all_posts_as_json)) 
        .route("/posts/new", post(api::add_post)) 
        .route("/posts/delete", delete(api::delete_post)) 
        .route("/posts/update", post(api::update_post))

        .route("/authors", get(api::fetch_all_authors_as_json))
        .route("/authors/new", post(api::add_author))
        .route("/authors/update", post(api::update_author))
        .route("/authors/:id", get(api::fetch_author_info_as_json))
        
        //All rendering goes here
        .route("/post", get(render::render_all_posts)) 
        .route("/post/:id", get(render::render_single_post)) 
        .route("/post/new", get(render::render_add_post_form))
        .route("/post/add_post", post(api::add_post))
        .layer(axum::extract::Extension(Arc::new(pool)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn initialize_database_tables(pool: &Pool<SqliteConnectionManager>) -> Result<(), rusqlite::Error> {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    
    db::create_author_table(&conn)?;
    db::create_posts_table(&conn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::Connection;

    fn create_in_memory_db_pool() -> Pool<SqliteConnectionManager> {
        let manager = SqliteConnectionManager::memory();
        Pool::new(manager).expect("Failed to create an in-memory database pool.")
    }

    #[tokio::test]
    async fn test_initialize_database_tables() {
        let pool = create_in_memory_db_pool();

        let init_result = initialize_database_tables(&pool).await;

        assert!(init_result.is_ok(), "Database tables initialization failed");

        let conn = pool.get().expect("Failed to get DB connection from pool");
        let tables_exist = check_tables_exist(&conn);
        assert!(tables_exist, "Not all required tables were created successfully");
    }

    fn check_tables_exist(conn: &Connection) -> bool {
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'").unwrap();
        let tables_iter = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();

        let mut tables = vec![];
        for table in tables_iter {
            tables.push(table.unwrap());
        }

        tables.contains(&"author".to_string()) && tables.contains(&"posts".to_string())
    }
}
