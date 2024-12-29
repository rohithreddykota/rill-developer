mod config;
mod credentials;
// todo: tidy up this code
mod examples;
use examples::org_projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    org_projects::org_projects().await?;
    Ok(())
}
