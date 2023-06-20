use sqlx::{Database, Error as SqlxError, Pool};

#[derive(Debug)]
pub enum DatabaseError {
    NotFound(String),
    CommunicationError,
    ConnectionFailed,
    ConnectionNotAvailable,
    QueryFailed(String),
    ColumnNotFound(String),
    ProtocolNotSupported,
    NotImplemented,
    Unknown(String),
    DatabaseInconsistence(String),
    MigrationFailed(String),
}

impl From<SqlxError> for DatabaseError {
    fn from(value: SqlxError) -> Self {
        println!("error: {:#?}", value);
        match value {
            SqlxError::ColumnNotFound(column_name) => Self::ColumnNotFound(column_name),
            SqlxError::Io(_) | SqlxError::Tls(_) => Self::CommunicationError,
            SqlxError::PoolTimedOut => Self::ConnectionNotAvailable,
            SqlxError::Database(e) => Self::QueryFailed(e.to_string()),
            SqlxError::Protocol(_) => Self::ProtocolNotSupported,
            SqlxError::TypeNotFound { type_name } => {
                Self::DatabaseInconsistence(format!("TypeNotFound {type_name}"))
            }
            _ => Self::ConnectionFailed,
        }
    }
}

#[async_trait::async_trait]
pub trait EntityRepository<
    DB: Database,
    Entity: Send,
    CreateInput: Send,
    UpdateInput: Send,
    QueryOne: Send + Sync,
    QueryMany: Send + Sync,
>
{
    async fn get(db: &Pool<DB>, key: QueryOne) -> Result<Entity, DatabaseError>;
    async fn try_get(db: &Pool<DB>, key: QueryOne) -> Result<Option<Entity>, DatabaseError>;
    async fn get_all(db: &Pool<DB>, key: QueryMany) -> Result<Vec<Entity>, DatabaseError>;
    async fn insert(db: &Pool<DB>, input: CreateInput) -> Result<Entity, DatabaseError>;
    async fn update(
        db: &Pool<DB>,
        key: QueryOne,
        input: UpdateInput,
    ) -> Result<Entity, DatabaseError>;
    async fn delete(db: &Pool<DB>, key: QueryOne) -> Result<Entity, DatabaseError>;
}
