use async_trait::async_trait;
use tracing::{debug, instrument};
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
    async fn get_jobs_by_queue(&self, queue: &str) -> Result<Vec<Job>, RustiqError>;
    async fn get_all_jobs(&self) -> Result<Vec<Job>, RustiqError>;
    async fn clear_queue(&self, queue: &str) -> Result<(), RustiqError>;
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
    #[instrument(skip(self))]
    async fn save_job(&self, job: &Job) -> Result<(), RustiqError> {
        debug!("Saving job: {}", job.id);
        let mut jobs = self.jobs.write().await;
        jobs.insert(job.id, job.clone());
        Ok(())
    }
    #[instrument(skip(self))]
    async fn get_job(&self, id: Uuid) -> Result<Option<Job>, RustiqError> {
        debug!("Retrieving job: {}", id);
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
    async fn get_jobs_by_queue(&self, queue: &str) -> Result<Vec<Job>, RustiqError> {
        let jobs = self.jobs.read().await;
        let filtered = jobs
            .values()
            .filter(|job| job.queue == queue)
            .cloned()
            .collect();
        Ok(filtered)
    }
    async fn get_all_jobs(&self) -> Result<Vec<Job>, RustiqError> {
        let jobs = self.jobs.read().await;
        Ok(jobs.values().cloned().collect())
    }
    async fn clear_queue(&self, queue: &str) -> Result<(), RustiqError> {
        let mut jobs = self.jobs.write().await;
        jobs.retain(|_, job| job.queue != queue);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_mock_storage_save_and_get() {
        let storage = MockStorage::new();
        let job = Job::new("test-queue", json!({"payload": 123}));
        let id = job.id;

        // Verify get returns None initially
        let get_init = storage.get_job(id).await.unwrap();
        assert!(get_init.is_none());

        // Save and verify get returns job
        storage.save_job(&job).await.unwrap();
        let get_after = storage.get_job(id).await.unwrap().unwrap();
        assert_eq!(get_after.id, id);
        assert_eq!(get_after.queue, "test-queue");
    }

    #[tokio::test]
    async fn test_mock_storage_delete() {
        let storage = MockStorage::new();
        let job = Job::new("test-queue", json!({}));
        let id = job.id;

        // Save job
        storage.save_job(&job).await.unwrap();

        // Delete job and check success
        storage.delete_job(id).await.unwrap();

        // Try to get deleted job, should be None
        assert!(storage.get_job(id).await.unwrap().is_none());

        // Try to delete again, should error with JobNotFound
        let err = storage.delete_job(id).await.unwrap_err();
        match err {
            RustiqError::JobNotFound(err_id) => assert_eq!(err_id, id),
            _ => panic!("Expected JobNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_mock_storage_update_status() {
        let storage = MockStorage::new();
        let job = Job::new("test-queue", json!({}));
        let id = job.id;

        // Save job
        storage.save_job(&job).await.unwrap();

        // Update status to Processing
        storage
            .update_job_status(id, JobStatus::Processing)
            .await
            .unwrap();

        // Verify status was updated
        let updated_job = storage.get_job(id).await.unwrap().unwrap();
        assert_eq!(updated_job.status, JobStatus::Processing);

        // Update status to non-existent job, should error
        let random_id = Uuid::new_v4();
        let err = storage
            .update_job_status(random_id, JobStatus::Done)
            .await
            .unwrap_err();
        match err {
            RustiqError::JobNotFound(err_id) => assert_eq!(err_id, random_id),
            _ => panic!("Expected JobNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_mock_storage_get_jobs_by_queue() {
        let storage = MockStorage::new();
        let job1 = Job::new("queue-a", json!({}));
        let job2 = Job::new("queue-a", json!({}));
        let job3 = Job::new("queue-b", json!({}));

        storage.save_job(&job1).await.unwrap();
        storage.save_job(&job2).await.unwrap();
        storage.save_job(&job3).await.unwrap();

        let jobs_a = storage.get_jobs_by_queue("queue-a").await.unwrap();
        assert_eq!(jobs_a.len(), 2);
        assert!(jobs_a.iter().any(|j| j.id == job1.id));
        assert!(jobs_a.iter().any(|j| j.id == job2.id));

        let jobs_b = storage.get_jobs_by_queue("queue-b").await.unwrap();
        assert_eq!(jobs_b.len(), 1);
        assert_eq!(jobs_b[0].id, job3.id);

        let jobs_c = storage.get_jobs_by_queue("queue-c").await.unwrap();
        assert!(jobs_c.is_empty());
    }

    #[tokio::test]
    async fn test_mock_storage_get_all_jobs() {
        let storage = MockStorage::new();
        let job1 = Job::new("queue-a", json!({}));
        let job2 = Job::new("queue-b", json!({}));

        storage.save_job(&job1).await.unwrap();
        storage.save_job(&job2).await.unwrap();

        let all = storage.get_all_jobs().await.unwrap();
        assert_eq!(all.len(), 2);
        assert!(all.iter().any(|j| j.id == job1.id));
        assert!(all.iter().any(|j| j.id == job2.id));
    }

    #[tokio::test]
    async fn test_mock_storage_clear_queue() {
        let storage = MockStorage::new();
        let job1 = Job::new("queue-a", json!({}));
        let job2 = Job::new("queue-a", json!({}));
        let job3 = Job::new("queue-b", json!({}));

        storage.save_job(&job1).await.unwrap();
        storage.save_job(&job2).await.unwrap();
        storage.save_job(&job3).await.unwrap();

        // Clear queue-a
        storage.clear_queue("queue-a").await.unwrap();

        // Verify queue-a is empty
        let jobs_a = storage.get_jobs_by_queue("queue-a").await.unwrap();
        assert!(jobs_a.is_empty());

        // Verify queue-b still has job3
        let jobs_b = storage.get_jobs_by_queue("queue-b").await.unwrap();
        assert_eq!(jobs_b.len(), 1);
        assert_eq!(jobs_b[0].id, job3.id);
    }
}
