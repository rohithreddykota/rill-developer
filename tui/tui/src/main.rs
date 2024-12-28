mod config;
mod credentials;
use admin::apis::admin_service_api as admin_api;
use config::ClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::default();

    let res = admin_api::admin_service_list_organizations(&config, None, None).await?;
    println!("{:?}", res);
    Ok(())
}
