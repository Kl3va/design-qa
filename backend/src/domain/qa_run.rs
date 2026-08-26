use chrono::{DateTime, Utc};
use uuid::{Uuid};
pub enum RunStatus {
 PENDING, RUNNING, COMPLETED, FAILED
}

pub struct QaRun {
 pub run_id: Uuid,
 pub project_id: Uuid,
 pub target_url: String,
 pub figma_file_key: String,
 pub figma_node_id: String,
 pub status: RunStatus,
 pub created_at: DateTime<Utc>,
 pub started_at: Option<DateTime<Utc>>,
 pub completed_at: Option<DateTime<Utc>>
}