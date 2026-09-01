use std::net::TcpListener;

use backend::startup;
//mod configuration::{get_configuration};
use backend::configuration::get_configuration;
use backend::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("design-qa".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to load configuration files");
    let listener = TcpListener::bind(&format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;
    //    let server = startup::run(listener).await?;
    //    server.await
    let application = startup::Application::build(listener, configuration).await?;
    application.run_until_stopped().await
}
