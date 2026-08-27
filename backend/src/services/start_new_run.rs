use sqlx::PgPool;

use crate::{domain::{NewQaRun, QaRun}, persistence::insert_qa_run};


pub async fn start_new_run(pool: &PgPool, new_run: NewQaRun) -> Result<QaRun, sqlx::Error> {
  let run = QaRun::new(new_run);
  insert_qa_run(pool, &run).await
}