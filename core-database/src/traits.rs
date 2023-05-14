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
}
