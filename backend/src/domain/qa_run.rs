use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{FigmaFileKey, FigmaNodeId, TargetUrl};

pub enum RunStatus {
    PENDING,
    RUNNING,
    COMPLETED,
    FAILED,
}

pub struct QaRun {
    pub run_id: Uuid,
    pub project_id: Uuid,
    pub target_url: TargetUrl,
    pub figma_file_key: FigmaFileKey,
    pub figma_node_id: FigmaNodeId,
    pub status: RunStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}
