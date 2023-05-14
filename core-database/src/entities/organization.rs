use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct OrganizationDAO {
    pub id: Uuid,
    pub name: String,
    pub active: bool
}