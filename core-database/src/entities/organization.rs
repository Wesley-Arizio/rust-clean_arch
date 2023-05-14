use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

#[derive(Debug)]
pub enum OrganizationBy {
    Id(Uuid),
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

#[derive(Debug)]
pub struct OrganizationRepository;

#[async_trait::async_trait]
impl
    EntityRepository<
        Sqlite,
        OrganizationDAO,
        OrganizationDAO,
        OrganizationDAO,
        OrganizationBy,
        OrganizationsWhere,
    > for OrganizationRepository
{
    async fn insert(
        db: &Pool<Sqlite>,
        input: OrganizationDAO,
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
        input: OrganizationDAO,
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
            }
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
        }
    }
}
