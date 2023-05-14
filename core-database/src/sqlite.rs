use crate::traits::DatabaseError;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Debug)]
pub struct DatabaseRepository {
    pub connection: Pool<Sqlite>,
}

impl DatabaseRepository {
    pub async fn new() -> Result<Self, DatabaseError> {
        let connection = SqlitePool::connect("sqlite::memory:")
            .await
            .map_err(DatabaseError::from)?;
        let repository = Self { connection };
        repository.migrate().await?;
        Ok(repository)
    }

    async fn migrate(&self) -> Result<sqlx::sqlite::SqliteQueryResult, DatabaseError> {
        let query = "
            CREATE TABLE organizations (
                id UUID NOT NULL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                active BOOLEAN NOT NULL DEFAULT true
            );

            CREATE TABLE admins (
                id UUID NOT NULL PRIMARY KEY,
                organization_id UUID NOT NULL,
                email TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                is_default BOOLEAN NOT NULL DEFAULT false,
                FOREIGN KEY (organization_id) REFERENCES organizations(id)
            );

            CREATE TABLE sellers (
                id UUID NOT NULL PRIMARY KEY,
                organization_id UUID NOT NULL,
                email TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                active BOOLEAN NOT NULL DEFAULT true,
                created_at INTEGER NO NULL DEFAULT (unixepoch('now')),
                FOREIGN KEY (organization_id) REFERENCES organizations(id)
            );

            CREATE TABLE products (
                id UUID NOT NULL PRIMARY KEY,
                organization_id UUID NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                amount INTEGER NOT NULL,
                price BLOB NOT NULL,
                created_at INTEGER NO NULL DEFAULT (unixepoch('now')),
                updated_at INTEGER NO NULL DEFAULT (unixepoch('now')),
                FOREIGN KEY (organization_id) REFERENCES organizations(id)
            );

            CREATE TABLE sales (
                id UUID NOT NULL PRIMARY KEY,
                product_id UUID NOT NULL,
                seller_id UUID NOT NULL,
                amount INTEGER NOT NULL,
                total_price BLOB NOT NULL,
                created_at INTEGER NO NULL DEFAULT (unixepoch('now')),
                updated_at INTEGER NO NULL DEFAULT (unixepoch('now')),
                FOREIGN KEY (product_id) REFERENCES products(id),
                FOREIGN KEY (seller_id) REFERENCES sellers(id)
            );
        ";
        let result: sqlx::sqlite::SqliteQueryResult = sqlx::query(&query)
            .execute(&self.connection)
            .await
            .map_err(DatabaseError::from)?;
        Ok(result)
    }
}
