mod config;
mod credentials;
// todo: tidy up this code
mod examples;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    examples::run().await?;
    Ok(())
}
