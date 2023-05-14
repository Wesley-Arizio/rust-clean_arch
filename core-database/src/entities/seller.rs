use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SellerDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}
