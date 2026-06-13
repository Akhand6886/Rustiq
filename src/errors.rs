use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustiqError {
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Queue not found: {0}")]
    QueueNotFound(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}
