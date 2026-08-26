//use backend::{configuration::get_configuration, startup::get_connection_pool};
use reqwest::StatusCode;

use crate::helpers::spawn_app;

mod helpers;

#[tokio::test]

async fn create_project_returns_201_for_valid_name() {
 let app = spawn_app().await;

 let client = reqwest::Client::new();

 let response = client.post(format!("{}/projects", app.address)).json(&serde_json::json!({"name": "My project"})).send().await.unwrap();

 assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn create_project_returns_400_bad_request_with_invalid_data() {
 let app = spawn_app().await;

 let client = reqwest::Client::new();

 let response = client.post(format!("{}/projects", app.address)).json(&serde_json::json!({"name":""})).send().await.unwrap();

 assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]

async fn create_project_confirms_data_exists_in_database () {
 let app = spawn_app().await;
 let client = reqwest::Client::new();

 let response = client.post(format!("{}/projects", app.address)).json(&serde_json::json!({"name": "New Project"})).send().await.unwrap();

 assert_eq!(response.status(), StatusCode::CREATED);

 let body:serde_json::Value = response.json().await.unwrap();
 let project_id = body["project_id"].as_str().unwrap();

// let config = get_configuration().unwrap();

// let pool = get_connection_pool(&app.settings.database);

 let result = sqlx::query!(r#"SELECT name, project_id FROM projects WHERE project_id = $1"#, uuid::Uuid::parse_str(project_id).unwrap()).fetch_one(&app.pool).await.unwrap();

 assert_eq!(result.name, "New Project");
}