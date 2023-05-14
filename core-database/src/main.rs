pub mod traits;
pub mod entities;
pub mod sqlite;

use entities::organization::OrganizationDAO;
use sqlite::DatabaseRepository;
use traits::{Repository, DatabaseError};
use uuid::Uuid;


#[async_trait::async_trait]
impl Repository for DatabaseRepository {
    async fn create_organization(&self, name: String) -> Result<OrganizationDAO, DatabaseError> {
        let uuid = Uuid::new_v4();
        sqlx::query_as::<_, OrganizationDAO>("INSERT INTO organizations (id, name) VALUES ($1, $2) RETURNING id, name, active")
            .bind(uuid)
            .bind(name)
            .fetch_one(&self.connection)
            .await
            .map_err(DatabaseError::from)
    }
}



#[tokio::main]
async fn main() -> Result<(), String> {
    let db = DatabaseRepository::new()
        .await
        .map_err(|e| format!("{:#?}", e))?;

    let result  = db.create_organization("Cryptize".to_string()).await.map_err(|e| format!("error: {:#?}", e))?;
    println!("Result: {:#?}", result);
    Ok(())
}
