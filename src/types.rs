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
}


