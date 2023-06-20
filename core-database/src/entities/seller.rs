use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

pub enum SellerBy {
    Id(Uuid),
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SellerDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct SellerRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, SellerDAO, SellerDAO, SellerDAO, SellerBy, SellerBy>
    for SellerRepository
{
    async fn insert(db: &Pool<Sqlite>, input: SellerDAO) -> Result<SellerDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        sqlx::query_as::<_, SellerDAO>(
            "INSERT INTO sales (id, organization_id, email, password) VALUES ($1, $2, $3, $4) RETURNING id, organization_id, email, password, active, created_at",
        )
        .bind(uuid)
        .bind(input.organization_id)
        .bind(input.email)
        .bind(input.password)
        .fetch_one(db)
        .await
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: SellerBy) -> Result<SellerDAO, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => sqlx::query_as::<_, SellerDAO>(
                "SELECT id, organization_id, email, password, active, created_at FROM sellers WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn try_get(db: &Pool<Sqlite>, key: SellerBy) -> Result<Option<SellerDAO>, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => sqlx::query_as::<_, SellerDAO>(
                "SELECT id, organization_id, email, password, active, created_at FROM sellers WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_optional(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(_db: &Pool<Sqlite>, _key: SellerBy) -> Result<Vec<SellerDAO>, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: SellerBy,
        input: SellerDAO,
    ) -> Result<SellerDAO, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => {
                sqlx::query_as::<_, SellerDAO>("UPDATE sellers SET password = $2, active = $3 WHERE id = $1 RETURNING id, organization_id, email, password, active, created_at")
                    .bind(uuid)
                    .bind(input.password)
                    .bind(input.active)
                    .fetch_one(db)
                    .await
                    .map_err(DatabaseError::from)
            }
        }
    }

    async fn delete(db: &Pool<Sqlite>, key: SellerBy) -> Result<SellerDAO, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => sqlx::query_as::<_, SellerDAO>(
                "DELETE FROM sellers WHERE id = $1 RETURNING id, organization_id, email, password, active, created_at",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
        }
    }
}
