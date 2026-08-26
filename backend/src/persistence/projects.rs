use crate::domain::{Project, ProjectName};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_project(pool: &PgPool, name: ProjectName) -> Result<Project, sqlx::Error> {
    let project_id = Uuid::new_v4();

    let record = sqlx::query!(r#"INSERT INTO projects (project_id, name) VALUES ($1, $2) RETURNING created_at, updated_at"#, project_id, name.as_ref()).fetch_one(pool).await?;

    Ok(Project {
        project_id,
        name,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}
