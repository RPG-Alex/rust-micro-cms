use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<SqliteConnectionManager>,
}

impl AppState {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> AppState {
        AppState {
            pool,
        }
    }
}
