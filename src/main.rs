use anyhow::{Ok, Result};
use recipena::HttpServer;

#[tokio::main]
async fn main() -> Result<()> {
    let config = recipena::config::load_config()?;
    recipena::logger::init_logger(&config)?;

    tracing::debug!("Starting Recipena");
    let server = HttpServer::new(config);
    server.run().await?;

    Ok(())
}
