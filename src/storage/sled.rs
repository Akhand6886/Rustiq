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
