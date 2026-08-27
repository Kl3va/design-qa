use crate::domain::RunStatus;
use crate::domain::QaRun;
use sqlx::PgPool;

pub async fn insert_qa_run(pool: &PgPool, run: &QaRun) -> Result<QaRun, sqlx::Error> {
    sqlx::query_as!(
        QaRun,
        r#"
        INSERT INTO qa_runs (
            run_id, project_id, target_url, figma_file_key,
            figma_node_id, status, created_at, started_at, completed_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
    .fetch_one(pool)
    .await
}