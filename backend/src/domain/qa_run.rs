use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use sqlx::Type;
use crate::domain::{NewQaRun};

#[derive(Debug, Clone, Copy, Type, Serialize, PartialEq)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, thiserror::Error)]
pub enum RunStateError {
    #[error("Cannot start a run with status {0:?}")]
    CannotStart(RunStatus),
    #[error("Cannot complete a run with status {0:?}")]
    CannotComplete(RunStatus),
    #[error("Cannot fail a run with status {0:?}")]
    CannotFail(RunStatus),
}

#[derive(Serialize)]
pub struct QaRun {
    pub run_id: Uuid,
    pub project_id: Uuid,
    pub target_url: String,
    pub figma_file_key: String,
    pub figma_node_id: String,
    status: RunStatus,
    pub created_at: DateTime<Utc>,
     started_at: Option<DateTime<Utc>>,
     completed_at: Option<DateTime<Utc>>,
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

 pub fn from_parts(
        run_id: Uuid,
        project_id: Uuid,
        target_url: String,
        figma_file_key: String,
        figma_node_id: String,
        status: RunStatus,
        created_at: DateTime<Utc>,
        started_at: Option<DateTime<Utc>>,
        completed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            run_id,
            project_id,
            target_url,
            figma_file_key,
            figma_node_id,
            status,
            created_at,
            started_at,
            completed_at,
        }
    }

 pub fn status (&self) -> RunStatus {
    self.status
 }

 pub fn completed_at (&self) -> Option<DateTime<Utc>> {
    self.completed_at
 }

 pub fn started_at (&self) -> Option<DateTime<Utc>> {
    self.started_at
 }

 pub fn start(&mut self) -> Result<(), RunStateError> {
    match self.status {
        RunStatus::Pending => {
            self.status = RunStatus::Running;
            self.started_at = Some(Utc::now());
            Ok(())
        }
        _ => Err(RunStateError::CannotStart(self.status)),
    }
 }

 pub fn complete (&mut self) -> Result<(), RunStateError> {
    match self.status {
        RunStatus::Running => {
            self.status = RunStatus::Completed;
            self.completed_at = Some(Utc::now());
            Ok(())
        }
        _ => Err(RunStateError::CannotComplete(self.status))
    }
 }

 pub fn fail (&mut self) -> Result<(), RunStateError> {
    match self.status {
        RunStatus::Running => {
            self.status = RunStatus::Failed;
            self.completed_at = Some(Utc::now());
            Ok(())
        }
        _ => Err(RunStateError::CannotFail(self.status))
    }
 }

}



#[cfg(test)]

mod test {
    use super::*;

    //This was intentional to bypass validation for the target_url,figma_file_key and the figma_nodeid
     fn test_run(status: RunStatus) -> QaRun {
        QaRun {
            run_id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            target_url: "https://example.com".to_string(),
            figma_file_key: "abc123".to_string(),
            figma_node_id: "1:1".to_string(),
            status,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }


    #[test]
    fn pending_cannot_complete() {
        let mut run = test_run(RunStatus::Pending);
        let result = run.complete();

        assert!(result.is_err());
        assert_eq!(run.status, RunStatus::Pending);
         
    }

    #[test] 
    fn pending_can_start () {
        let mut run = test_run(RunStatus::Pending);
        let result = run.start();

        assert!(result.is_ok());
        assert_eq!(run.status, RunStatus::Running);
        assert!(run.started_at().is_some());
    }

     #[test]
    fn transition_from_pending_to_completed() {
        let mut run = test_run(RunStatus::Pending);

        assert!(run.start().is_ok());
        assert_eq!(run.status(), RunStatus::Running);

        assert!(run.complete().is_ok());
        assert_eq!(run.status(), RunStatus::Completed);
    }

    #[test]
fn transition_from_running_to_failed() {
    let mut run = test_run(RunStatus::Pending);

    run.start().unwrap();

    assert!(run.fail().is_ok());
    assert_eq!(run.status(), RunStatus::Failed);
    assert!(run.completed_at().is_some());
}


}