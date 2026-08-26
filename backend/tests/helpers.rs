use std::net::TcpListener;

use backend::{configuration::{DatabaseSettings, Settings, get_configuration}, startup::{Application}};
use sqlx::{Connection, PgConnection, PgPool, Executor};

//#[derive(Display)]
#[allow(dead_code)]
pub struct TestApp {
 pub address: String,
 pub pool: PgPool,
 pub settings: Settings,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut config = get_configuration().unwrap();
    config.database.database_name = uuid::Uuid::new_v4().to_string();
  //  let pool = get_connection_pool(&config.database);
  let pool = configure_database(&config.database).await;
    let application = Application::build(listener, config.clone()).await.unwrap();

    let address = format!("http://127.0.0.1:{}", application.port());

    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
     address,
     pool,
     settings: config,
    }
}


async fn configure_database (config: &DatabaseSettings) -> PgPool {

  // Connect to Postgres server (without specifying a database)
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    // Create the test database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect to the new database and run migrations
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool

}