use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;

use crate::configuration::Settings;
use zaoraj_rs::routes::{complain, health_check};

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            config.application.host,
            config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = HttpServer::new(move || {
            App::new()
                // .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/complain", web::post().to(complain))
        })
            .listen(listener)?
            .run();

        Ok(Self {port, server})
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}