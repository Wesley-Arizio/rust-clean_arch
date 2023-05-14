use chrono::{DateTime, Utc};
use num_bigint::BigUint;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

pub enum ProductBy {
    Id(Uuid),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProductDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: String,
    pub amount: u32,
    pub price: BigUint,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SqliteProductDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: String,
    pub amount: i32,
    pub price: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ProductDAO> for SqliteProductDAO {
    fn from(value: ProductDAO) -> Self {
        Self {
            id: value.id,
            organization_id: value.organization_id,
            name: value.name,
            amount: i32::try_from(value.amount).unwrap_or_default(),
            price: value.price.to_bytes_le(),
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<SqliteProductDAO> for ProductDAO {
    fn from(value: SqliteProductDAO) -> Self {
        Self {
            id: value.id,
            organization_id: value.organization_id,
            description: value.description,
            name: value.name,
            amount: value.amount.unsigned_abs(),
            price: BigUint::from_bytes_le(&value.price),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct ProductRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, ProductDAO, ProductDAO, ProductDAO, ProductBy, ProductBy>
    for ProductRepository
{
    async fn insert(db: &Pool<Sqlite>, input: ProductDAO) -> Result<ProductDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        let input = SqliteProductDAO::from(input);
        sqlx::query_as::<_, SqliteProductDAO>(
            "INSERT INTO products (id, organization_id, name, description, amount, price) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, organization_id, name, description, amount, price, created_at, updated_at",
        )
        .bind(uuid)
        .bind(input.organization_id)
        .bind(input.name)
        .bind(input.description)
        .bind(input.amount)
        .bind(input.price)
        .fetch_one(db)
        .await
        .map(ProductDAO::from)
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: ProductBy) -> Result<ProductDAO, DatabaseError> {
        match key {
            ProductBy::Id(uuid) => sqlx::query_as::<_, SqliteProductDAO>(
                "SELECT id, organization_id, name, description, amount, price, created_at, updated_at FROM products WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map(ProductDAO::from)
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(
        _db: &Pool<Sqlite>,
        _key: ProductBy,
    ) -> Result<Vec<ProductDAO>, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: ProductBy,
        input: ProductDAO,
    ) -> Result<ProductDAO, DatabaseError> {
        let input = SqliteProductDAO::from(input);
        match key {
            ProductBy::Id(uuid) => {
                sqlx::query_as::<_, SqliteProductDAO>("UPDATE products SET name = $2, description = $3, amount = $4, price = $5, updated_at = unixepoch('now') WHERE id = $1 RETURNING id, organization_id, name, description, amount, price, created_at, updated_at")
                    .bind(uuid)
                    .bind(input.name)
                    .bind(input.description)
                    .bind(input.amount)
                    .bind(input.price)
                    .fetch_one(db)
                    .await
                    .map(ProductDAO::from)
                    .map_err(DatabaseError::from)
            }
        }
    }

    async fn delete(db: &Pool<Sqlite>, key: ProductBy) -> Result<ProductDAO, DatabaseError> {
        match key {
            ProductBy::Id(uuid) => sqlx::query_as::<_, SqliteProductDAO>(
                "DELETE FROM products WHERE id = $1 RETURNING id, organization_id, name, description, amount, price, created_at, updated_at",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map(ProductDAO::from)
            .map_err(DatabaseError::from),
        }
    }
}
