use actix_web::{HttpResponse, ResponseError, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::{domain::ProjectName, persistence::create_project_query, utils::error_chain_fmt};

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
 project_id: Uuid,
 name: String,
 created_at: DateTime<Utc>,
 updated_at: DateTime<Utc>
}

#[derive(thiserror::Error)]
pub enum CreateProjectError {
 #[error("{0}")]
 InvalidName(String),

 #[error("Failed to create project")]
 Database(#[from] sqlx::Error)
}

impl std::fmt::Debug for CreateProjectError {
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     error_chain_fmt(self, f)
 }
}

impl ResponseError for CreateProjectError {
 fn status_code(&self) -> actix_web::http::StatusCode {
     match self {
      Self::InvalidName(_) => StatusCode::BAD_REQUEST,
      Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR
     }
 }
}

pub async fn create_project (request: web::Json<CreateProjectRequest>, pool: web::Data<PgPool>) -> Result<HttpResponse, CreateProjectError> {
  let project_name = ProjectName::parse(request.0.name).map_err(CreateProjectError::InvalidName)?;

  let project = create_project_query(&pool, project_name).await?;

   Ok(HttpResponse::Ok().json(CreateProjectResponse {
    project_id: project.project_id,
    name: project.name.as_ref().to_string(),
    created_at: project.created_at,
    updated_at: project.updated_at
   }))
}
