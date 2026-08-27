use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{create_project, create_run};
use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::health_check,
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        listener: TcpListener,
        configuration: Settings,
    ) -> Result<Self, std::io::Error> {
        let port = listener.local_addr()?.port();
        let connection_pool = get_connection_pool(&configuration.database);
        let server = run(listener, connection_pool).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

//Create a Pool via sqlx
pub fn get_connection_pool(connection: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(connection.with_db())
}

pub async fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(connection_pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/projects", web::post().to(create_project)).route("/projects/{project_id}/runs", web::post().to(create_run))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
