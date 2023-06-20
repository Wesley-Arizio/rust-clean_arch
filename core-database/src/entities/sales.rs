use chrono::{DateTime, Utc};
use num_bigint::BigUint;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::traits::{DatabaseError, EntityRepository};

pub enum SalesBy {
    Id(Uuid),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SalesDAO {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub amount: u32,
    pub total_price: BigUint,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewSalesDAO {
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub amount: u32,
    pub total_price: BigUint,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UpdateSalesDAO {
    pub amount: u32,
    pub total_price: BigUint,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SqliteSalesDAO {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub amount: i32,
    pub total_price: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SalesDAO> for SqliteSalesDAO {
    fn from(value: SalesDAO) -> Self {
        Self {
            id: value.id,
            product_id: value.product_id,
            seller_id: value.seller_id,
            amount: i32::try_from(value.amount).unwrap_or_default(),
            total_price: value.total_price.to_bytes_le(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<NewSalesDAO> for SqliteSalesDAO {
    fn from(value: NewSalesDAO) -> Self {
        Self {
            id: Uuid::default(),
            product_id: value.product_id,
            seller_id: value.seller_id,
            amount: i32::try_from(value.amount).unwrap_or_default(),
            total_price: value.total_price.to_bytes_le(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<UpdateSalesDAO> for SqliteSalesDAO {
    fn from(value: UpdateSalesDAO) -> Self {
        Self {
            id: Uuid::default(),
            product_id: Uuid::default(),
            seller_id: Uuid::default(),
            amount: i32::try_from(value.amount).unwrap_or_default(),
            total_price: value.total_price.to_bytes_le(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<SqliteSalesDAO> for SalesDAO {
    fn from(value: SqliteSalesDAO) -> Self {
        Self {
            id: value.id,
            product_id: value.product_id,
            seller_id: value.seller_id,
            amount: value.amount.unsigned_abs(),
            total_price: BigUint::from_bytes_le(&value.total_price),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct SalesRepository;

#[async_trait::async_trait]
impl EntityRepository<Sqlite, SalesDAO, NewSalesDAO, UpdateSalesDAO, SalesBy, SalesBy>
    for SalesRepository
{
    async fn insert(db: &Pool<Sqlite>, input: NewSalesDAO) -> Result<SalesDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        let input: SqliteSalesDAO = SqliteSalesDAO::from(input);
        sqlx::query_as::<_, SqliteSalesDAO>(
            "INSERT INTO sales (id, product_id, seller_id, amount, total_price) VALUES ($1, $2, $3, $4, $5) RETURNING id, product_id, seller_id, amount, total_price, created_at, updated_at",
        )
        .bind(uuid)
        .bind(input.product_id)
        .bind(input.seller_id)
        .bind(input.amount)
        .bind(input.total_price)
        .fetch_one(db)
        .await
        .map(SalesDAO::from)
        .map_err(DatabaseError::from)
    }

    async fn get(db: &Pool<Sqlite>, key: SalesBy) -> Result<SalesDAO, DatabaseError> {
        match key {
            SalesBy::Id(uuid) => sqlx::query_as::<_, SqliteSalesDAO>(
                "SELECT id, product_id, seller_id, amount, total_price, created_at, updated_at FROM sales WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map(SalesDAO::from)
            .map_err(DatabaseError::from),
        }
    }

    async fn try_get(db: &Pool<Sqlite>, key: SalesBy) -> Result<Option<SalesDAO>, DatabaseError> {
        match key {
            SalesBy::Id(uuid) => sqlx::query_as::<_, SqliteSalesDAO>(
                "SELECT id, product_id, seller_id, amount, total_price, created_at, updated_at FROM sales WHERE id = $1 LIMIT 1",
            )
            .bind(uuid)
            .fetch_optional(db)
            .await
            .map(|v| v.map(SalesDAO::from))
            .map_err(DatabaseError::from),
        }
    }

    async fn get_all(_db: &Pool<Sqlite>, _key: SalesBy) -> Result<Vec<SalesDAO>, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    async fn update(
        db: &Pool<Sqlite>,
        key: SalesBy,
        input: UpdateSalesDAO,
    ) -> Result<SalesDAO, DatabaseError> {
        let input = SqliteSalesDAO::from(input);
        match key {
            SalesBy::Id(uuid) => {
                sqlx::query_as::<_, SqliteSalesDAO>("UPDATE sales SET amount = $2, total_price = $3, updated_at = unixepoch('now') WHERE id = $1 RETURNING id, product_id, seller_id, amount, total_price, created_at, updated_at")
                    .bind(uuid)
                    .bind(input.amount)
                    .bind(input.total_price)
                    .fetch_one(db)
                    .await
                    .map(SalesDAO::from)
                    .map_err(DatabaseError::from)
            }
        }
    }

    async fn delete(db: &Pool<Sqlite>, key: SalesBy) -> Result<SalesDAO, DatabaseError> {
        match key {
            SalesBy::Id(uuid) => sqlx::query_as::<_, SqliteSalesDAO>(
                "DELETE FROM sales WHERE id = $1 RETURNING id, product_id, seller_id, amount, total_price, created_at, updated_at",
            )
            .bind(uuid)
            .fetch_one(db)
            .await
            .map(SalesDAO::from)
            .map_err(DatabaseError::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use crate::{
        entities::{
            organization::{NewOrganizationDAO, OrganizationDAO, OrganizationRepository},
            product::{NewProductDAO, ProductRepository},
            seller::{NewSellerDAO, SellerRepository},
        },
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

        let product = ProductRepository::insert(
            &db.connection,
            NewProductDAO {
                organization_id: organization.id,
                name: "Iphone".to_string(),
                description: "smartphone".to_string(),
                amount: 10,
                price: BigUint::from(5000u32),
            },
        )
        .await
        .expect("Could not create a new product");

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

        let sales = SalesRepository::insert(
            &db.connection,
            NewSalesDAO {
                product_id: product.id,
                seller_id: seller.id,
                amount: 2,
                total_price: BigUint::from(product.price.mul(2u32)),
            },
        )
        .await
        .expect("Could not create a new sale");

        let sales = SalesRepository::get(&db.connection, SalesBy::Id(sales.id))
            .await
            .expect("Could not get sales");
        assert_eq!(sales.amount, 2);
        assert_eq!(sales.product_id, product.id);
        assert_eq!(sales.seller_id, seller.id);

        let sales = SalesRepository::try_get(&db.connection, SalesBy::Id(sales.id))
            .await
            .expect("Could not get sales")
            .unwrap();
        assert_eq!(sales.amount, 2);
        assert_eq!(sales.product_id, product.id);
        assert_eq!(sales.seller_id, seller.id);

        let maybe_sales = SalesRepository::try_get(&db.connection, SalesBy::Id(Uuid::default()))
            .await
            .expect("Could not get sales");
        assert!(maybe_sales.is_none());

        let updated = SalesRepository::update(
            &db.connection,
            SalesBy::Id(sales.id),
            UpdateSalesDAO {
                amount: 4,
                total_price: BigUint::from(34u32),
            },
        )
        .await
        .expect("Could not get sales");
        assert_eq!(updated.amount, 4);
        assert_eq!(updated.total_price, BigUint::from(34u32));
        assert_eq!(updated.product_id, product.id);
        assert_eq!(updated.seller_id, seller.id);

        let deleted = SalesRepository::delete(&db.connection, SalesBy::Id(sales.id))
            .await
            .expect("Could not get sales");

        let maybe_sales = SalesRepository::try_get(&db.connection, SalesBy::Id(deleted.id))
            .await
            .expect("Could not get sales");
        assert!(maybe_sales.is_none());
    }
}
