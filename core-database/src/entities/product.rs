use num_bigint::BigUint;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProductDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: String,
    pub amount: u32,
    pub price: BigUint,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
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
    pub updated_at: DateTime<Utc>
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
            updated_at: value.updated_at
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
            updated_at: value.updated_at
        }
    }
}