use crate::errors::RustiqError;

/// A persistent storage backend using `sled`.
pub struct SledStorage {
    db: sled::Db,
}

impl SledStorage {
    /// Initializes a new SledStorage instance at the given path.
    pub fn new(path: &str) -> Result<Self, RustiqError> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sled_storage_initialization() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_str().unwrap();

        let storage_result = SledStorage::new(path);
        assert!(storage_result.is_ok(), "Failed to initialize SledStorage");
        
        let storage = storage_result.unwrap();
        // Verifying we can successfully flush to confirm DB is active and writable
        let flush_result = storage.db.flush();
        assert!(flush_result.is_ok(), "Failed to flush Sled DB");
    }
}
