use sqlx::{Pool, Sqlite, SqlitePool};

use crate::traits::DatabaseError;

#[derive(Debug)]
pub struct DatabaseRepository {
    pub connection: Pool<Sqlite>
}

impl DatabaseRepository {
    pub async fn new() -> Result<Self, DatabaseError> {
        let connection = SqlitePool::connect("sqlite::memory:").await.map_err(DatabaseError::from)?;
        let repository = Self {
            connection
        };
        repository.migrate().await?;
        Ok(repository)
    }

    async fn migrate(&self) -> Result<sqlx::sqlite::SqliteQueryResult, DatabaseError> {
        let query = "
            CREATE TABLE organizations (
                id UUID NOT NULL PRIMARY KEY,
                name TEXT NOT NULL,
                active BOOLEAN NOT NULL DEFAULT true
            );
        ";
        let result: sqlx::sqlite::SqliteQueryResult = sqlx::query(&query).execute(&self.connection).await.map_err(DatabaseError::from)?;
        Ok(result)
    }
}