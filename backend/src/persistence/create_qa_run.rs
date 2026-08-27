use sqlx::PgPool;

use crate::domain::{QaRun};



pub async fn insert_qa_run(pool: &PgPool, run: &QaRun) -> Result<QaRun, sqlx::Error> {
  let row = sqlx::query!(r#"INSERT INTO qa_runs (run_id, project_id, target_url, figma_file_key,
            figma_node_id, status, created_at, started_at, completed_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
            run_id, project_id, target_url, figma_file_key, figma_node_id,
            status AS "status: RunStatus",
            created_at, started_at, completed_at"#,)

            
}