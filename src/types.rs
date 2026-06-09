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
}


