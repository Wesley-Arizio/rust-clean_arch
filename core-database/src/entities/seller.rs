use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

pub enum SellerBy {
    Id(Uuid),
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SellerDAO {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct NewSellerDAO {
    pub organization_id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct UpdateSellerDAO {
    pub password: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct SellerRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, SellerDAO, NewSellerDAO, UpdateSellerDAO, SellerBy, SellerBy>
    for SellerRepository
{
    async fn insert(db: &Pool<Sqlite>, input: NewSellerDAO) -> Result<SellerDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        sqlx::query_as::<_, SellerDAO>(
            "INSERT INTO sellers (id, organization_id, email, password) VALUES ($1, $2, $3, $4) RETURNING id, organization_id, email, password, active, created_at",
        )
        .bind(uuid)
        .bind(input.organization_id)
        .bind(input.email)
        .bind(input.password)
        .fetch_one(db)
        .await
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: SellerBy) -> Result<SellerDAO, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => sqlx::query_as::<_, SellerDAO>(
                "SELECT id, organization_id, email, password, active, created_at FROM sellers WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn try_get(db: &Pool<Sqlite>, key: SellerBy) -> Result<Option<SellerDAO>, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => sqlx::query_as::<_, SellerDAO>(
                "SELECT id, organization_id, email, password, active, created_at FROM sellers WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_optional(db)
            .await
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(_db: &Pool<Sqlite>, _key: SellerBy) -> Result<Vec<SellerDAO>, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: SellerBy,
        input: UpdateSellerDAO,
    ) -> Result<SellerDAO, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => {
                sqlx::query_as::<_, SellerDAO>("UPDATE sellers SET password = $2, active = $3 WHERE id = $1 RETURNING id, organization_id, email, password, active, created_at")
                    .bind(uuid)
                    .bind(input.password)
                    .bind(input.active)
                    .fetch_one(db)
                    .await
                    .map_err(DatabaseError::from)
            }
        }
    }

    async fn delete(db: &Pool<Sqlite>, key: SellerBy) -> Result<SellerDAO, DatabaseError> {
        match key {
            SellerBy::Id(uuid) => sqlx::query_as::<_, SellerDAO>(
                "DELETE FROM sellers WHERE id = $1 RETURNING id, organization_id, email, password, active, created_at",
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
    use crate::{
        entities::organization::{NewOrganizationDAO, OrganizationDAO, OrganizationRepository},
        sqlite::DatabaseRepository,
    };

    use super::*;

    async fn create_organization(pool: &Pool<Sqlite>, name: &str) -> OrganizationDAO {
        OrganizationRepository::insert(
            pool,
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

        let organization = create_organization(&db.connection, "test").await;

        let seller = SellerRepository::insert(
            &db.connection,
            NewSellerDAO {
                organization_id: organization.id,
                email: "test@gmail.com".to_string(),
                password: "test123".to_string(),
            },
        )
        .await
        .expect("Could not create a seller");

        let seller = SellerRepository::get(&db.connection, SellerBy::Id(seller.id))
            .await
            .expect("Could not find seller");

        assert_eq!(seller.organization_id, organization.id);
        assert_eq!(seller.email, "test@gmail.com");
        assert_eq!(seller.password, "test123");
        assert!(seller.active);

        let seller = SellerRepository::try_get(&db.connection, SellerBy::Id(seller.id))
            .await
            .expect("Could not find seller")
            .unwrap();

        assert_eq!(seller.organization_id, organization.id);
        assert_eq!(seller.email, "test@gmail.com");
        assert_eq!(seller.password, "test123");
        assert!(seller.active);

        let updated = SellerRepository::update(
            &db.connection,
            SellerBy::Id(seller.id),
            UpdateSellerDAO {
                password: "newpassword".to_string(),
                active: false,
            },
        )
        .await
        .expect("Could not find seller");

        assert_eq!(updated.password, "newpassword");
        assert!(!updated.active);

        let deleted = SellerRepository::delete(&db.connection, SellerBy::Id(seller.id))
            .await
            .expect("Could not delete seller");

        let maybe_seller = SellerRepository::try_get(&db.connection, SellerBy::Id(deleted.id))
            .await
            .expect("Could not find seller");

        assert!(maybe_seller.is_none());
    }
}
