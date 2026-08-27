use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use sqlx::Type;
use crate::domain::{NewQaRun};

#[derive(Debug, Clone, Copy, Type, Serialize)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize)]
pub struct QaRun {
    pub run_id: Uuid,
    pub project_id: Uuid,
    pub target_url: String,
    pub figma_file_key: String,
    pub figma_node_id: String,
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
   target_url: run.target_url.as_ref().to_string(),
   figma_file_key: run.figma_file_key.as_ref().to_string(),
   figma_node_id: run.figma_node_id.as_ref().to_string(),
   status: RunStatus::Pending,
   created_at: Utc::now(),
   started_at: None,
   completed_at: None
  }
 }
}