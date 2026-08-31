use reqwest::StatusCode;

use crate::helpers::spawn_app;

mod helpers;

#[tokio::test]

async fn create_qa_run () {

 let app = spawn_app().await;
 let client = reqwest::Client::new();

 let project_response = client.post(format!("{}/projects", app.address)).json(&serde_json::json!({"name":"Testing"})).send().await.unwrap();

 assert_eq!(project_response.status(), StatusCode::CREATED);

 let body: serde_json::Value = project_response.json().await.unwrap();

 let project_id = body["project_id"].as_str().unwrap();

 //create a qa_run
 let qa_response = client.post(format!("{}/projects/{}/runs",app.address, project_id)).json(&serde_json::json!({
  "target_url": "https://example.com",
    "figma_file_key": "aBcD1234EfGh5678IjKl",
    "figma_node_id": "2345:2366"
 })).send().await.unwrap();
 
 assert_eq!(qa_response.status(), StatusCode::CREATED);

 let qa_body:serde_json::Value = qa_response.json().await.unwrap();
 
 assert_eq!(qa_body["target_url"].as_str().unwrap(), "https://example.com");
 assert_eq!(qa_body["figma_file_key"].as_str().unwrap(), "aBcD1234EfGh5678IjKl");
 assert_eq!(qa_body["figma_node_id"].as_str().unwrap(), "2345:2366");

 assert_eq!(project_id, qa_body["project_id"].as_str().unwrap());
 assert_eq!(qa_body["status"].as_str().unwrap(), "pending");
 assert!(qa_body["started_at"].is_null());
 assert!(uuid::Uuid::parse_str(qa_body["run_id"].as_str().unwrap()).is_ok());

}



#[tokio::test]
async fn return_bad_request_for_invalid_input () {

  let app = spawn_app().await;

  let client = reqwest::Client::new();

  let response = client.post(format!("{}/projects", app.address)).json(&serde_json::json!({"name":"New test project"})).send().await.unwrap();

  assert_eq!(response.status(), StatusCode::CREATED);

  let body: serde_json::Value = response.json().await.unwrap();
  let project_id = body["project_id"].as_str().unwrap();

  let qa_response = client.post(format!("{}/projects/{}/runs", app.address, project_id)).json(&serde_json::json!({"target_url": "https://example.com",
    "figma_file_key": "file-key",
    "figma_node_id": "node-id"})).send().await.unwrap();

  assert_eq!(qa_response.status(), StatusCode::BAD_REQUEST);
} 