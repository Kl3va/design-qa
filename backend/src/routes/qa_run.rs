use actix_web::{HttpResponse, ResponseError, http::StatusCode, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::{NewQaRun, NewQaRunError}, services::start_new_run, utils::error_chain_fmt};

#[derive(Deserialize)]
pub struct CreateRunRequest {
  //  pub project_id: Uuid,
    pub target_url: String,
    pub figma_file_key: String,
    pub figma_node_id: String,
}

#[derive(thiserror::Error)]
pub enum CreateRunError {
 #[error("{0}")]
 InvalidPayload(#[from] NewQaRunError),
  #[error("Failed to create run")]
    Database(#[from] sqlx::Error)
}

impl std::fmt::Debug for CreateRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreateRunError {
 fn status_code(&self) -> actix_web::http::StatusCode {
     match self {
      Self::InvalidPayload(_) => StatusCode::BAD_REQUEST,
      Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR
     }
 }
}

pub async fn create_run (path: web::Path<Uuid>, req: web::Json<CreateRunRequest>, pool: web::Data<PgPool>) -> Result<HttpResponse, CreateRunError> {
  let project_id = path.into_inner();
  let new_run: NewQaRun = (req.into_inner(), project_id).try_into()?;

  let run = start_new_run(&pool, new_run).await?;

  Ok(HttpResponse::Created().json(run))
}