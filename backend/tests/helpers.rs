use std::net::TcpListener;

use backend::startup::Application;

pub async fn spawn_app() -> String {
 let listener = TcpListener::bind("127.0.0.1:0").unwrap();
 let application = Application::build(listener).await.unwrap();

 let address = format!("http://127.0.0.1:{}", application.port());

 let _ = tokio::spawn(application.run_until_stopped());

 address
} 