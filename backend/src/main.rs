use std::net::TcpListener;

use backend::startup;
mod configuration;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = configuration::get_configuration().expect("Failed to load configuration files");
   let listener = TcpListener::bind(&format!("{}:{}", configuration.application.host, configuration.application.port))?;
//    let server = startup::run(listener).await?;
//    server.await
let application = startup::Application::build(listener).await?;
application.run_until_stopped().await

}