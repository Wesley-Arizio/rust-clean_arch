use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AdminDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
    pub is_default: bool,
}
