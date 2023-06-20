use core_database::{
    entities::organization::{OrganizationBy, OrganizationDAO, OrganizationRepository},
    sqlite::DatabaseRepository,
    traits::EntityRepository,
};
use uuid::Uuid;

pub async fn create_organization(
    db: &DatabaseRepository,
    name: String,
) -> Result<OrganizationDAO, String> {
    let maybe_organization =
        OrganizationRepository::try_get(&db.connection, OrganizationBy::Name(name.clone()))
            .await
            .map_err(|e| format!("database error: {:#?}", e))?;

    if let Some(_) = maybe_organization {
        return Err("Organization aready exists".to_string());
    };

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
