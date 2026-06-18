use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::RustiqError;
use crate::types::{Job, JobStatus};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_job(&self, job: &Job) -> Result<(), RustiqError>;
}


