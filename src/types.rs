use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Processing,
    Done,
    Failed,
    DeadLetter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub queue: String,
    pub payload: Value,
    pub status: JobStatus,
    pub priority: u8,
    pub retry_count: u32,
    pub max_retries: u32,
    pub created_at: DateTime<Utc>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub visibility_timeout_secs: u64,
    pub result: Option<Value>,
    pub error: Option<String>,
}

impl Job {
    pub fn new(queue: impl Into<String>, payload: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            queue: queue.into(),
            payload,
            status: JobStatus::Queued,
            priority: 0,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
            scheduled_at: None,
            lease_expires_at: None,
            visibility_timeout_secs: 30,
            result: None,
            error: None,
        }
    }

    pub fn is_queued(&self) -> bool {
        self.status == JobStatus::Queued
    }

    pub fn is_processing(&self) -> bool {
        self.status == JobStatus::Processing
    }

    pub fn is_done(&self) -> bool {
        self.status == JobStatus::Done
    }

    pub fn is_failed(&self) -> bool {
        self.status == JobStatus::Failed
    }

    pub fn is_dead_letter(&self) -> bool {
        self.status == JobStatus::DeadLetter
    }

    pub fn is_lease_expired(&self) -> bool {
        if let Some(expires_at) = self.lease_expires_at {
            expires_at < Utc::now()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueueConfig {
    pub visibility_timeout_secs: u64,
    pub max_retries: u32,
    pub max_concurrency: Option<usize>,
    pub dead_letter_queue: Option<String>,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            visibility_timeout_secs: 30,
            max_retries: 3,
            max_concurrency: None,
            dead_letter_queue: None,
        }
    }
}

pub struct QueueConfigBuilder {
    visibility_timeout_secs: u64,
    max_retries: u32,
    max_concurrency: Option<usize>,
    dead_letter_queue: Option<String>,
}

impl QueueConfig {
    pub fn builder() -> QueueConfigBuilder {
        QueueConfigBuilder {
            visibility_timeout_secs: 30,
            max_retries: 3,
            max_concurrency: None,
            dead_letter_queue: None,
        }
    }
}

impl QueueConfigBuilder {
    pub fn visibility_timeout_secs(mut self, secs: u64) -> Self {
        self.visibility_timeout_secs = secs;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn max_concurrency(mut self, concurrency: Option<usize>) -> Self {
        self.max_concurrency = concurrency;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_status_serialization() {
        assert_eq!(serde_json::to_string(&JobStatus::Queued).unwrap(), "\"queued\"");
        assert_eq!(serde_json::to_string(&JobStatus::Processing).unwrap(), "\"processing\"");
        assert_eq!(serde_json::to_string(&JobStatus::Done).unwrap(), "\"done\"");
        assert_eq!(serde_json::to_string(&JobStatus::Failed).unwrap(), "\"failed\"");
        assert_eq!(serde_json::to_string(&JobStatus::DeadLetter).unwrap(), "\"dead_letter\"");
    }

    #[test]
    fn test_job_status_deserialization() {
        let status: JobStatus = serde_json::from_str("\"queued\"").unwrap();
        assert_eq!(status, JobStatus::Queued);

        let status: JobStatus = serde_json::from_str("\"dead_letter\"").unwrap();
        assert_eq!(status, JobStatus::DeadLetter);
    }

    #[test]
    fn test_job_serialization() {
        let job = Job {
            id: Uuid::nil(),
            queue: "default".to_string(),
            payload: serde_json::json!({"data": "hello"}),
            status: JobStatus::Queued,
            priority: 1,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
            scheduled_at: None,
            lease_expires_at: None,
            visibility_timeout_secs: 30,
            result: None,
            error: None,
        };

        let serialized = serde_json::to_string(&job).unwrap();
        assert!(serialized.contains("\"queue\":\"default\""));
        assert!(serialized.contains("\"status\":\"queued\""));
    }

    #[test]
    fn test_job_deserialization() {
        let json_data = r#"{
            "id": "00000000-0000-0000-0000-000000000000",
            "queue": "default",
            "payload": {"data": "hello"},
            "status": "queued",
            "priority": 1,
            "retry_count": 0,
            "max_retries": 3,
            "created_at": "2026-06-09T07:28:00Z",
            "scheduled_at": null,
            "lease_expires_at": null,
            "visibility_timeout_secs": 30,
            "result": null,
            "error": null
        }"#;

        let job: Job = serde_json::from_str(json_data).unwrap();
        assert_eq!(job.id, Uuid::nil());
        assert_eq!(job.queue, "default");
        assert_eq!(job.status, JobStatus::Queued);
        assert_eq!(job.scheduled_at, None);
        assert_eq!(job.lease_expires_at, None);
        assert_eq!(job.result, None);
        assert_eq!(job.error, None);
    }

    #[test]
    fn test_job_new_defaults() {
        let payload = serde_json::json!({"task": "test"});
        let job = Job::new("emails", payload.clone());

        assert_eq!(job.queue, "emails");
        assert_eq!(job.payload, payload);
        assert_eq!(job.status, JobStatus::Queued);
        assert_eq!(job.priority, 0);
        assert_eq!(job.retry_count, 0);
        assert_eq!(job.max_retries, 3);
        assert_eq!(job.visibility_timeout_secs, 30);
        assert_eq!(job.scheduled_at, None);
        assert_eq!(job.lease_expires_at, None);
        assert_eq!(job.result, None);
        assert_eq!(job.error, None);
    }

    #[test]
    fn test_job_helper_methods() {
        let mut job = Job::new("default", serde_json::json!({}));
        assert!(job.is_queued());
        assert!(!job.is_processing());

        job.status = JobStatus::Processing;
        assert!(job.is_processing());

        job.status = JobStatus::Done;
        assert!(job.is_done());

        job.status = JobStatus::Failed;
        assert!(job.is_failed());

        job.status = JobStatus::DeadLetter;
        assert!(job.is_dead_letter());

        assert!(!job.is_lease_expired());
        
        job.lease_expires_at = Some(Utc::now() - chrono::Duration::seconds(10));
        assert!(job.is_lease_expired());

        job.lease_expires_at = Some(Utc::now() + chrono::Duration::seconds(10));
        assert!(!job.is_lease_expired());
    }

    #[test]
    fn test_queue_config_default() {
        let config = QueueConfig::default();
        assert_eq!(config.visibility_timeout_secs, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.max_concurrency, None);
        assert_eq!(config.dead_letter_queue, None);
    }
}


