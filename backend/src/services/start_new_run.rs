use sqlx::PgPool;

use crate::{domain::{NewQaRun, QaRun}, persistence::{InsertQaRunError, insert_qa_run}};


#[tracing::instrument(
  name = "start a new qa run",
  skip(pool, new_run)
)]
pub async fn start_new_run(pool: &PgPool, new_run: NewQaRun) -> Result<QaRun, InsertQaRunError> {
  let run = QaRun::new(new_run);
  insert_qa_run(pool, &run).await
}