use crate::app::Application;
use crate::configuration::get_configuration;

mod app;
mod configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let server = Application::build(config).await?;
    server.run_until_stopped().await?;
    Ok(())
}
