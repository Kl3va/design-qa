use actix_web::{App, web, HttpServer};
use std::net::TcpListener;
use actix_web::dev::Server;

use crate::routes::health_check;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
 let server = HttpServer::new(|| {
  App::new().route("/health_check", web::get().to(health_check))
 }).listen(listener)?.run();

 Ok(server)
}