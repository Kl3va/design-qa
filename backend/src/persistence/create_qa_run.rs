use crate::domain::RunStatus;
use crate::domain::QaRun;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum InsertQaRunError {
    #[error("project with id {0} not found")]
    ProjectNotFound(Uuid),

    #[error("Database error")]
    Database(#[from] sqlx::Error)
}
#[tracing::instrument(
    name= "insert a qa run to the database",
    skip(pool, run)
)]

pub async fn insert_qa_run(pool: &PgPool, run: &QaRun) -> Result<QaRun, InsertQaRunError> {
    let result = sqlx::query_as!(
        QaRun,
        r#"
        
        INSERT INTO qa_runs (
            run_id, project_id, target_url, figma_file_key,
            figma_node_id, status, created_at, started_at, completed_at
        )
        SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9
        WHERE EXISTS (SELECT 1 FROM projects WHERE project_id = $2)
        RETURNING
            run_id, project_id, target_url, figma_file_key, figma_node_id,
            status AS "status: _",
            created_at, started_at, completed_at
        "#,
        run.run_id,
        run.project_id,
        run.target_url,
        run.figma_file_key,
        run.figma_node_id,
        run.status as RunStatus,
        run.created_at,
        run.started_at,
        run.completed_at,
    )
    .fetch_optional(pool)
    .await?;

    result.ok_or(InsertQaRunError::ProjectNotFound(run.project_id))
}


// pub async fn find_project_id (pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
//     sqlx::query!(r#""#)
// }