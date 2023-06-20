use chrono::{DateTime, Utc};
use num_bigint::BigUint;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

pub enum SalesBy {
    Id(Uuid),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SalesDAO {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub amount: u32,
    pub total_price: BigUint,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SqliteSalesDAO {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub amount: i32,
    pub total_price: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SalesDAO> for SqliteSalesDAO {
    fn from(value: SalesDAO) -> Self {
        Self {
            id: value.id,
            product_id: value.product_id,
            seller_id: value.seller_id,
            amount: i32::try_from(value.amount).unwrap_or_default(),
            total_price: value.total_price.to_bytes_le(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<SqliteSalesDAO> for SalesDAO {
    fn from(value: SqliteSalesDAO) -> Self {
        Self {
            id: value.id,
            product_id: value.product_id,
            seller_id: value.seller_id,
            amount: value.amount.unsigned_abs(),
            total_price: BigUint::from_bytes_le(&value.total_price),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct SalesRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, SalesDAO, SalesDAO, SalesDAO, SalesBy, SalesBy> for SalesRepository {
    async fn insert(db: &Pool<Sqlite>, input: SalesDAO) -> Result<SalesDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        let input: SqliteSalesDAO = SqliteSalesDAO::from(input);
        sqlx::query_as::<_, SqliteSalesDAO>(
            "INSERT INTO sales (id, product_id, seller_id, amount, total_price) VALUES ($1, $2, $3, $4, $5) RETURNING id, product_id, seller_id, amount, total_price, created_at, updated_at",
        )
        .bind(uuid)
        .bind(input.product_id)
        .bind(input.seller_id)
        .bind(input.amount)
        .bind(input.total_price)
        .fetch_one(db)
        .await
        .map(SalesDAO::from)
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: SalesBy) -> Result<SalesDAO, DatabaseError> {
        match key {
            SalesBy::Id(uuid) => sqlx::query_as::<_, SqliteSalesDAO>(
                "SELECT id, product_id, seller_id, amount, total_price, created_at, updated_at FROM sales WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map(SalesDAO::from)
            .map_err(DatabaseError::from),
        }
    }

    async fn try_get(db: &Pool<Sqlite>, key: SalesBy) -> Result<Option<SalesDAO>, DatabaseError> {
        match key {
            SalesBy::Id(uuid) => sqlx::query_as::<_, SqliteSalesDAO>(
                "SELECT id, product_id, seller_id, amount, total_price, created_at, updated_at FROM sales WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_optional(db)
            .await
            .map(|v| v.map(SalesDAO::from))
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(_db: &Pool<Sqlite>, _key: SalesBy) -> Result<Vec<SalesDAO>, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: SalesBy,
        input: SalesDAO,
    ) -> Result<SalesDAO, DatabaseError> {
        let input = SqliteSalesDAO::from(input);
        match key {
            SalesBy::Id(uuid) => {
                sqlx::query_as::<_, SqliteSalesDAO>("UPDATE sales SET amount = $2, total_price = $3, updated_at = unixepoch('now') WHERE id = $1 RETURNING id, product_id, seller_id, amount, total_price, created_at, updated_at")
                    .bind(uuid)
                    .bind(input.amount)
                    .bind(input.total_price)
                    .fetch_one(db)
                    .await
                    .map(SalesDAO::from)
                    .map_err(DatabaseError::from)
            }
        }
    }

    async fn delete(db: &Pool<Sqlite>, key: SalesBy) -> Result<SalesDAO, DatabaseError> {
        match key {
            SalesBy::Id(uuid) => sqlx::query_as::<_, SqliteSalesDAO>(
                "DELETE FROM sales WHERE id = $1 RETURNING id, product_id, seller_id, amount, total_price, created_at, updated_at",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map(SalesDAO::from)
            .map_err(DatabaseError::from),
        }
    }
}
