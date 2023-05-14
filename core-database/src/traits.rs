use sqlx::{Error as SqlxError};
use crate::entities::organization::OrganizationDAO;


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
			},
			_ => Self::ConnectionFailed,
        }
    }
}

#[async_trait::async_trait]
pub trait Repository {
    async fn create_organization(&self, name: String) -> Result<OrganizationDAO, DatabaseError>;
}