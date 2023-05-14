use core_database::{
    entities::organization::{OrganizationDAO, OrganizationRepository},
    sqlite::DatabaseRepository,
    traits::EntityRepository,
};
use uuid::Uuid;

pub async fn create_organization(
    db: &DatabaseRepository,
    name: String,
) -> Result<OrganizationDAO, String> {
    // TODO - validate organization name before creating it
    OrganizationRepository::insert(
        &db.connection,
        OrganizationDAO {
            id: Uuid::default(),
            name,
            active: bool::default(),
        },
    )
    .await
    .map_err(|e| format!("database error: {:#?}", e))
}
