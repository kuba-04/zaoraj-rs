use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::Prompt;

use zaoraj_rs::routes::{complain, health_check};

use crate::configuration::Settings;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let openai_config = OpenAIConfig::new()
            .with_api_key(config.openai.api_key)
            .with_api_base(config.openai.base_url);
        let openai_client = web::Data::new(Client::with_config(openai_config));
        let prompt = web::Data::new(Prompt::from(config.openai.prompt));

        let server = HttpServer::new(move || {
            App::new()
                //todo .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/complain", web::post().to(complain))
                .app_data(openai_client.clone())
                .app_data(prompt.clone())
        })
            .listen(listener)?
            .run();

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}