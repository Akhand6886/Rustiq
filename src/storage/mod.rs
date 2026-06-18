use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::RustiqError;
use crate::types::{Job, JobStatus};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_job(&self, job: &Job) -> Result<(), RustiqError>;
    async fn get_job(&self, id: Uuid) -> Result<Option<Job>, RustiqError>;
    async fn delete_job(&self, id: Uuid) -> Result<(), RustiqError>;
    async fn update_job_status(&self, id: Uuid, status: JobStatus) -> Result<(), RustiqError>;
}


