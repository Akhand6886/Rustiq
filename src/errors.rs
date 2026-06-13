use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustiqError {
    #[error("Storage error: {0}")]
    StorageError(String),
}
