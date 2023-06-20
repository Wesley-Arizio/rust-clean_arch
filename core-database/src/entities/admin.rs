use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

#[derive(Debug)]
pub enum AdminBy {
    Id(Uuid),
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AdminDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
    pub is_default: bool,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct NewAdminDAO {
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
    pub is_default: bool,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct UpdateAdminDAO {
    pub password: String,
    pub is_default: bool,
}

impl From<AdminDAO> for NewAdminDAO {
    fn from(value: AdminDAO) -> Self {
        Self {
            organization_id: value.organization_id,
            email: value.email,
            password: value.password,
            is_default: value.is_default,
        }
    }
}

#[derive(Debug)]
pub struct AdminRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, AdminDAO, NewAdminDAO, UpdateAdminDAO, AdminBy, AdminBy>
    for AdminRepository
{
    async fn insert(db: &Pool<Sqlite>, input: NewAdminDAO) -> Result<AdminDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        sqlx::query_as::<_, AdminDAO>(
            "INSERT INTO admins (id, organization_id, email, password, is_default) VALUES ($1, $2, $3, $4, $5) RETURNING id, organization_id, email, password, is_default",
        )
        .bind(uuid)
        .bind(input.organization_id)
        .bind(input.email)
        .bind(input.password)
        .bind(input.is_default)
        .fetch_one(db)
        .await
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: AdminBy) -> Result<AdminDAO, DatabaseError> {
        match key {
            AdminBy::Id(uuid) => sqlx::query_as::<_, AdminDAO>(
                "SELECT id, organization_id, email, password, is_default FROM admins WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn try_get(db: &Pool<Sqlite>, key: AdminBy) -> Result<Option<AdminDAO>, DatabaseError> {
        match key {
            AdminBy::Id(uuid) => sqlx::query_as::<_, AdminDAO>(
                "SELECT id, organization_id, email, password, is_default FROM admins WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_optional(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(_db: &Pool<Sqlite>, _key: AdminBy) -> Result<Vec<AdminDAO>, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: AdminBy,
        input: UpdateAdminDAO,
    ) -> Result<AdminDAO, DatabaseError> {
        match key {
            AdminBy::Id(uuid) => {
                sqlx::query_as::<_, AdminDAO>("UPDATE admins SET password = $2, is_default = $3 WHERE id = $1 RETURNING id, organization_id, email, password, is_default")
                    .bind(uuid)
                    .bind(input.password)
                    .bind(input.is_default)
                    .fetch_one(db)
                    .await
                    .map_err(DatabaseError::from)
            }
        }
    }

    async fn delete(db: &Pool<Sqlite>, key: AdminBy) -> Result<AdminDAO, DatabaseError> {
        match key {
            AdminBy::Id(uuid) => sqlx::query_as::<_, AdminDAO>(
                "DELETE FROM admins WHERE id = $1 RETURNING id, organization_id, email, password, is_default",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::organization::{NewOrganizationDAO, OrganizationDAO, OrganizationRepository},
        sqlite::DatabaseRepository,
    };

    async fn create_organization(conection: &Pool<Sqlite>, name: &str) -> OrganizationDAO {
        OrganizationRepository::insert(
            conection,
            NewOrganizationDAO {
                name: name.to_string(),
            },
        )
        .await
        .expect("Could not create organization")
    }

    #[tokio::test]
    async fn queries() {
        let db = DatabaseRepository::new()
            .await
            .expect("Could not initialize db");

        let organization = create_organization(&db.connection, "dev").await;
        let result = AdminRepository::insert(
            &db.connection,
            NewAdminDAO {
                organization_id: organization.id,
                email: "admin@gmail.com".to_string(),
                password: "test1".to_string(),
                is_default: false,
            },
        )
        .await
        .expect("Could not insert admin");

        assert_eq!(result.email, "admin@gmail.com");
        assert_eq!(result.password, "test1");
        assert_eq!(result.organization_id, organization.id);
        assert!(!result.is_default);

        let admin = AdminRepository::get(&db.connection, AdminBy::Id(result.id))
            .await
            .expect("Admin not found");
        assert_eq!(result.email, admin.email);

        let admin = AdminRepository::try_get(&db.connection, AdminBy::Id(result.id))
            .await
            .expect("Admin not found");

        assert!(admin.is_some());

        let maybe_admin = AdminRepository::try_get(&db.connection, AdminBy::Id(Uuid::default()))
            .await
            .expect("Could not get admin info");

        assert!(maybe_admin.is_none());

        let updated = AdminRepository::update(
            &db.connection,
            AdminBy::Id(result.id),
            UpdateAdminDAO {
                password: "test34".to_string(),
                is_default: true,
            },
        )
        .await
        .expect("Could not update admin info");

        assert_eq!(updated.id, result.id);
        assert_eq!(updated.email, "admin@gmail.com");
        assert_ne!(updated.password, result.password);
        assert!(updated.is_default);

        let _ = AdminRepository::delete(&db.connection, AdminBy::Id(result.id))
            .await
            .expect("Could not delete admin");

        let admin = AdminRepository::try_get(&db.connection, AdminBy::Id(result.id))
            .await
            .expect("Admin not found");

        assert!(admin.is_none());
    }
}
