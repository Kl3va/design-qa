use std::net::TcpListener;
mod configuration;
mod startup;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = configuration::get_configuration().expect("Failed to load configuration files");
   let listener = TcpListener::bind(&format!("{}:{}", configuration.application.host, configuration.application.port))?;
   let server = startup::run(listener).await?;
   server.await
}