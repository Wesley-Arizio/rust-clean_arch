use crate::traits::DatabaseError;
use sqlx::{
    migrate::{Migrate, Migrator},
    Pool, Sqlite, SqlitePool,
};

#[derive(Debug)]
pub struct DatabaseRepository {
    pub connection: Pool<Sqlite>,
}

static MIGRATOR: Migrator = sqlx::migrate!("./sqlite-migrations");

impl DatabaseRepository {
    pub async fn new() -> Result<Self, DatabaseError> {
        let connection = SqlitePool::connect("sqlite::memory:")
            .await
            .map_err(DatabaseError::from)?;

        DatabaseRepository::migrate(&connection).await?;
        let repository: DatabaseRepository = Self { connection };
        Ok(repository)
    }

    async fn migrate(pool: &Pool<Sqlite>) -> Result<(), DatabaseError> {
        let mut conn = pool.acquire().await.map_err(DatabaseError::from)?;
        conn.ensure_migrations_table()
            .await
            .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))?;
        for migration in MIGRATOR.iter() {
            if migration.migration_type.is_down_migration() {
                // Skipping down migrations
                continue;
            }
            conn.apply(migration)
                .await
                .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))?;
        }

        Ok(())
    }
}
