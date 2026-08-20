use std::net::TcpListener;

use backend::startup::Application;

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


}