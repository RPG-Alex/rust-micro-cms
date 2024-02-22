use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, sqlite::SqlitePool};
use anyhow::{Result, Context};

pub struct DBConnection {
    pub pool: Pool<Sqlite>,
}

impl DBConnection {
    pub async fn new(db_path: &str) -> Result<Self> {
        DBConnection::initialize_db(db_path).await?;
        DBConnection::connect(db_path).await
    }

    pub async fn connect(db_path: &str) -> Result<Self> {
        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path))
            .await
            .with_context(|| format!("Could not connect to database at '{}'", db_path))?;
        Ok(DBConnection { pool })
    }

    async fn initialize_db(db_path: &str) -> Result<()> {
        if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
            println!("Creating database {}", db_path);
            Sqlite::create_database(db_path).await
                .with_context(|| format!("Failed to create database at '{}'", db_path))?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_in_memory_db() -> Result<DBConnection> {
        let db_path = "sqlite::memory:";
        DBConnection::new(db_path).await
    }

    #[tokio::test]
    async fn test_initialize_and_connect_in_memory_db() -> Result<()> {
        let connection_result = setup_in_memory_db().await;
        assert!(connection_result.is_ok());

        Ok(())
    }

}
