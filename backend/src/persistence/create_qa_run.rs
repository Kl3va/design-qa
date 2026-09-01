use crate::domain::RunStatus;
use crate::domain::QaRun;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum InsertQaRunError {
    #[error("project with id {0} not found")]
    ProjectNotFound(Uuid),

    #[error("Failed to insert QA run")]
    Database(#[from] sqlx::Error)
}
#[tracing::instrument(skip_all)]

pub async fn insert_qa_run(pool: &PgPool, run: &QaRun) -> Result<QaRun, InsertQaRunError> {
    let result = sqlx::query!(
        
        r#"
        
        INSERT INTO qa_runs (
            run_id, project_id, target_url, figma_file_key,
            figma_node_id, status, created_at, started_at, completed_at
        )
        SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9
        WHERE EXISTS (SELECT 1 FROM projects WHERE project_id = $2)
        RETURNING
            run_id, project_id, target_url, figma_file_key, figma_node_id,
            status AS "status: RunStatus",
            created_at, started_at, completed_at
        "#,
        run.run_id,
        run.project_id,
        run.target_url,
        run.figma_file_key,
        run.figma_node_id,
        run.status() as RunStatus,
        run.created_at,
        run.started_at(),
        run.completed_at(),
    )
    .fetch_optional(pool)
    .await?;

   let row = result.ok_or(InsertQaRunError::ProjectNotFound(run.project_id))?;

    Ok(QaRun::from_parts(
        row.run_id,
        row.project_id,
        row.target_url,
        row.figma_file_key,
        row.figma_node_id,
        row.status,
        row.created_at,
        row.started_at,
        row.completed_at,
    ))
}