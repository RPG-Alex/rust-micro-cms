use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<SqliteConnectionManager>,
}

impl AppState {
    pub fn new(db_path: &str) -> Self {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(manager).expect("Failed to create pool.");
        AppState { pool }
    }
}
