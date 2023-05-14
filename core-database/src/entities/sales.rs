use chrono::{DateTime, Utc};
use num_bigint::BigUint;
use uuid::Uuid;

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
