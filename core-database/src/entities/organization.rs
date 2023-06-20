use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

#[derive(Debug)]
pub enum OrganizationBy {
    Id(Uuid),
    Name(String),
}

#[derive(Debug)]
pub enum OrganizationsWhere {
    Active {
        active: bool,
        limit: i32,
        offset: i32,
    },
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct OrganizationDAO {
    pub id: Uuid,
    pub name: String,
    pub active: bool,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct NewOrganizationDAO {
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct UpdateOrganizationDAO {
    pub name: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct OrganizationRepository;

#[async_trait::async_trait]
impl
    EntityRepository<
        Sqlite,
        OrganizationDAO,
        NewOrganizationDAO,
        UpdateOrganizationDAO,
        OrganizationBy,
        OrganizationsWhere,
    > for OrganizationRepository
{
    async fn insert(
        db: &Pool<Sqlite>,
        input: NewOrganizationDAO,
    ) -> Result<OrganizationDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        sqlx::query_as::<_, OrganizationDAO>(
            "INSERT INTO organizations (id, name) VALUES ($1, $2) RETURNING id, name, active",
        )
        .bind(uuid)
        .bind(input.name)
        .fetch_one(db)
        .await
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: OrganizationBy) -> Result<OrganizationDAO, DatabaseError> {
        match key {
            OrganizationBy::Id(uuid) => sqlx::query_as::<_, OrganizationDAO>(
                "SELECT id, name, active FROM organizations WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
            OrganizationBy::Name(name) => sqlx::query_as::<_, OrganizationDAO>(
                "SELECT id, name, active FROM organizations WHERE name = $1 LIMIT 1",
            )
            .bind(name)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn try_get(
        db: &Pool<Sqlite>,
        key: OrganizationBy,
    ) -> Result<Option<OrganizationDAO>, DatabaseError> {
        match key {
            OrganizationBy::Id(uuid) => sqlx::query_as::<_, OrganizationDAO>(
                "SELECT id, name, active FROM organizations WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_optional(db)
            .await
            .map_err(DatabaseError::from),
            OrganizationBy::Name(name) => sqlx::query_as::<_, OrganizationDAO>(
                "SELECT id, name, active FROM organizations WHERE name = $1 LIMIT 1",
            )
            .bind(name)
            .fetch_optional(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(
        db: &Pool<Sqlite>,
        key: OrganizationsWhere,
    ) -> Result<Vec<OrganizationDAO>, DatabaseError> {
        match key {
            OrganizationsWhere::Active {
                active,
                limit,
                offset,
            } => sqlx::query_as::<_, OrganizationDAO>(
                "SELECT id, name, active FROM organizations WHERE active = $1 LIMIT $2 OFFSET $3",
            )
            .bind(active)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: OrganizationBy,
        input: UpdateOrganizationDAO,
    ) -> Result<OrganizationDAO, DatabaseError> {
        match key {
            OrganizationBy::Id(uuid) => {
                sqlx::query_as::<_, OrganizationDAO>("UPDATE organizations SET name = $2, active = $3 WHERE id = $1 RETURNING id, name, active")
                    .bind(uuid)
                    .bind(input.name)
                    .bind(input.active)
                    .fetch_one(db)
                    .await
                    .map_err(DatabaseError::from)
            },
            OrganizationBy::Name(_) => Err(DatabaseError::NotImplemented),
        }
    }

    async fn delete(
        db: &Pool<Sqlite>,
        key: OrganizationBy,
    ) -> Result<OrganizationDAO, DatabaseError> {
        match key {
            OrganizationBy::Id(uuid) => sqlx::query_as::<_, OrganizationDAO>(
                "DELETE FROM organizations WHERE id = $1 RETURNING id, name, active",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
            OrganizationBy::Name(_) => Err(DatabaseError::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sqlite::DatabaseRepository;

    #[tokio::test]
    async fn queries() {
        let db = DatabaseRepository::new()
            .await
            .expect("Could not initialize db");

        let organization = OrganizationRepository::insert(
            &db.connection,
            NewOrganizationDAO {
                name: "dev3".to_string(),
            },
        )
        .await
        .expect("Could not create organization");

        assert!(organization.active);
        assert_eq!(organization.name, "dev3");

        let organization =
            OrganizationRepository::get(&db.connection, OrganizationBy::Id(organization.id))
                .await
                .expect("Could not find organization");

        assert!(organization.active);
        assert_eq!(organization.name, "dev3");

        let organization =
            OrganizationRepository::get(&db.connection, OrganizationBy::Name(organization.name))
                .await
                .expect("Could not find organization");

        assert!(organization.active);
        assert_eq!(organization.name, "dev3");

        let maybe_organization =
            OrganizationRepository::try_get(&db.connection, OrganizationBy::Id(organization.id))
                .await
                .expect("Could not find organization");

        assert!(maybe_organization.is_some());

        let maybe_organization =
            OrganizationRepository::try_get(&db.connection, OrganizationBy::Id(Uuid::default()))
                .await
                .expect("Could not find organization");

        assert!(maybe_organization.is_none());

        let updated = OrganizationRepository::update(
            &db.connection,
            OrganizationBy::Id(organization.id),
            UpdateOrganizationDAO {
                name: "dev4".to_string(),
                active: false,
            },
        )
        .await
        .expect("Could not update organization by id");

        assert_eq!(updated.name, "dev4");
        assert_ne!(updated.name, organization.name);
        assert!(!updated.active);

        let updated = OrganizationRepository::update(
            &db.connection,
            OrganizationBy::Name(updated.name),
            UpdateOrganizationDAO {
                name: "dev45".to_string(),
                active: true,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(updated, DatabaseError::NotImplemented);

        let _ = OrganizationRepository::delete(&db.connection, OrganizationBy::Id(organization.id))
            .await
            .expect("Could not delete organization by id");

        let maybe_organization =
            OrganizationRepository::try_get(&db.connection, OrganizationBy::Id(organization.id))
                .await
                .expect("Could not find organization");

        assert!(maybe_organization.is_none());

        let deleted =
            OrganizationRepository::delete(&db.connection, OrganizationBy::Name(organization.name))
                .await
                .unwrap_err();
        assert_eq!(deleted, DatabaseError::NotImplemented);
    }
}
