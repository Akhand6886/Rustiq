use thiserror::Error;

/// Central error type for the Rustiq task queue system.
#[derive(Debug, Error)]
pub enum RustiqError {
    /// Errors arising from DB operations or file access.
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Requested queue does not exist.
    #[error("Queue not found: {0}")]
    QueueNotFound(String),

    /// Failed to serialize or deserialize JSON payload.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Supplied job payload violates size or validation criteria.
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    /// Specified job identifier could not be retrieved from DB.
    #[error("Job not found: {0}")]
    JobNotFound(uuid::Uuid),
}

impl From<serde_json::Error> for RustiqError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}
