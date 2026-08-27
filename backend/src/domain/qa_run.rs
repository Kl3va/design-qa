use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{FigmaFileKey, FigmaNodeId, NewQaRun, TargetUrl};

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

impl QaRun {

 pub fn new(run:NewQaRun) -> Self {
  Self {
   run_id: uuid::Uuid::new_v4(),
   project_id: run.project_id,
   target_url: run.target_url,
   figma_file_key: run.figma_file_key,
   figma_node_id: run.figma_node_id,
   status: RunStatus::PENDING,
   created_at: Utc::now(),
   started_at: None,
   completed_at: None
  }
 }
}