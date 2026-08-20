use actix_web::{App, web, HttpServer};
use std::net::TcpListener;
use actix_web::dev::Server;

use crate::routes::health_check;

pub struct Application {
 port: u16,
 server: Server
}

impl Application {
 pub async fn build(listener: TcpListener) -> Result<Self, std::io::Error> {
       let port = listener.local_addr()?.port();
       let server = run(listener).await?;
       Ok(Self { port, server })
 }

 pub fn port(&self) -> u16 {
  self.port
 }

 pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
   self.server.await
 }
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
 let server = HttpServer::new(|| {
  App::new().route("/health_check", web::get().to(health_check))
 }).listen(listener)?.run();

 Ok(server)
}