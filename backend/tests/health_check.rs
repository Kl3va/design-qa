use std::net::TcpListener;

use backend::startup::Application;
use reqwest::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();

    let application = Application::build(listener).await.unwrap();

    let address = format!("http://127.0.0.1:{}", application.port());

    tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.status(), StatusCode::OK);


}


#[tokio::test]

async fn non_existing_route_returns_404 () {
 let listener = TcpListener::bind("127.0.0.1:0").unwrap();
 let application = Application::build(listener).await.unwrap();
 let address = format!("http://127.0.0.1:{}", application.port());

 tokio::spawn(application.run_until_stopped());
 let client = reqwest::Client::new();

 let response = client.get(format!("{}/exit", address)).send().await.unwrap();

 assert_eq!(response.status(), StatusCode::NOT_FOUND);
}