use crate::domain::ProjectName;
use chrono::{DateTime, Utc};
use uuid::Uuid;

//#[derive(Debug)]
pub struct Project {
    pub project_id: Uuid,
    pub name: ProjectName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
