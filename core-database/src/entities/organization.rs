use sqlx::{Database, Pool};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

#[derive(Debug)]
pub enum OrganizationBy {
    Id(Uuid),
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct OrganizationDAO {
    pub id: Uuid,
    pub name: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct OrganizationRepository;

#[async_trait::async_trait]
impl<DB: Database>
    EntityRepository<
        DB,
        OrganizationDAO,
        OrganizationDAO,
        OrganizationDAO,
        OrganizationBy,
        OrganizationBy,
    > for OrganizationRepository
{
    async fn get(_db: &Pool<DB>, _key: OrganizationBy) -> Result<OrganizationDAO, DatabaseError> {
        Ok(OrganizationDAO {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            active: true,
        })
    }
}
