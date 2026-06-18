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

#[async_trait]
impl Storage for MockStorage {
    async fn save_job(&self, job: &Job) -> Result<(), RustiqError> {
        let mut jobs = self.jobs.write().await;
        jobs.insert(job.id, job.clone());
        Ok(())
    }
    async fn get_job(&self, id: Uuid) -> Result<Option<Job>, RustiqError> {
        let jobs = self.jobs.read().await;
        Ok(jobs.get(&id).cloned())
    }
    async fn delete_job(&self, id: Uuid) -> Result<(), RustiqError> {
        let mut jobs = self.jobs.write().await;
        if jobs.remove(&id).is_some() {
            Ok(())
        } else {
            Err(RustiqError::JobNotFound(id))
        }
    }
    async fn update_job_status(&self, id: Uuid, status: JobStatus) -> Result<(), RustiqError> {
        let mut jobs = self.jobs.write().await;
        if let Some(job) = jobs.get_mut(&id) {
            job.status = status;
            Ok(())
        } else {
            Err(RustiqError::JobNotFound(id))
        }
    }
}#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
}
