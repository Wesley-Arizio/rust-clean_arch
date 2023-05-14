use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SalesDAO {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub commission_per_sale: u32,
    pub amount: u32,
    pub description: String,
    // TODO - Change it to BigUint
    pub total_price: String,
    pub discount: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}