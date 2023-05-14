use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct ProductDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub amount: u32,
    pub description: String,
    // TODO - Change it to BigUint
    pub price: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}