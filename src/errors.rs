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

impl From<uuid::Error> for RustiqError {
    fn from(err: uuid::Error) -> Self {
        Self::InvalidPayload(format!("Invalid UUID: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json_error_conversion() {
        let invalid_json = "{ invalid }";
        let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        let rustiq_err: RustiqError = err.into();
        
        match rustiq_err {
            RustiqError::SerializationError(msg) => {
                assert!(msg.contains("key") || msg.contains("expected") || msg.contains("line 1"));
            }
            _ => panic!("Expected SerializationError"),
        }
    }
}
