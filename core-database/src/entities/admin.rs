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

#[derive(Debug)]
pub struct AdminRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, AdminDAO, AdminDAO, AdminDAO, AdminBy, AdminBy> for AdminRepository {
    async fn insert(db: &Pool<Sqlite>, input: AdminDAO) -> Result<AdminDAO, DatabaseError> {
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
        input: AdminDAO,
    ) -> Result<AdminDAO, DatabaseError> {
        match key {
            AdminBy::Id(uuid) => {
                sqlx::query_as::<_, AdminDAO>("UPDATE admins SET password = $2 WHERE id = $1 RETURNING id, organization_id, email, password, is_default")
                    .bind(uuid)
                    .bind(input.password)
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
