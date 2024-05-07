use tokio_rusqlite::{Connection, Result};

pub struct AppState {
    conn: Connection,
}

impl AppState {
    pub async fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path).await?;
        Ok(Self { conn })
    }
}
