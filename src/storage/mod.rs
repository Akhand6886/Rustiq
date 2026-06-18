use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::RustiqError;
use crate::types::{Job, JobStatus};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_job(&self, job: &Job) -> Result<(), RustiqError>;
    async fn get_job(&self, id: Uuid) -> Result<Option<Job>, RustiqError>;
    async fn delete_job(&self, id: Uuid) -> Result<(), RustiqError>;
    async fn update_job_status(&self, id: Uuid, status: JobStatus) -> Result<(), RustiqError>;
}

#[derive(Debug, Clone)]
pub struct MockStorage {
    jobs: Arc<RwLock<HashMap<Uuid, Job>>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MockStorage {
    fn default() -> Self {
        Self::new()
    }
}




