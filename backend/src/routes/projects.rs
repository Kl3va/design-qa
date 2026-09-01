use actix_web::{HttpResponse, ResponseError, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::ProjectName, persistence::insert_project, utils::error_chain_fmt};

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    pub project_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(thiserror::Error)]
pub enum CreateProjectError {
    #[error("{0}")]
    InvalidName(String),

    #[error("Failed to create project")]
    Database(#[from] sqlx::Error),
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
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Create a project",
    skip(request, pool)
)]

pub async fn create_project(
    request: web::Json<CreateProjectRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CreateProjectError> {
    let CreateProjectRequest { name } = request.into_inner();
    let project_name = ProjectName::parse(name).map_err(CreateProjectError::InvalidName)?;

    let project = insert_project(&pool, project_name).await?;

    Ok(HttpResponse::Created().json(CreateProjectResponse {
        project_id: project.project_id,
        name: project.name.as_ref().to_string(),
        created_at: project.created_at,
        updated_at: project.updated_at,
    }))
}
