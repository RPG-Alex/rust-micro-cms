
use sqlx::SqlitePool;

pub struct AppState {
    pub db: SqlitePool,
}
