use crate::config::ClientConfig;
use admin::apis::admin_service_api as admin_api;

pub async fn org_projects() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::default();
    let orgs = admin_api::admin_service_list_organizations(&config, None, None).await?;
    for org in orgs.organizations.into_iter().flatten() {
        if let Some(name) = org.name {
            println!();
            println!("Organization: {}", name);
            let projects = admin_api::admin_service_list_projects_for_organization(
                &config,
                name.as_str(),
                None,
                None,
            )
            .await?;
            for project in projects.projects.into_iter().flatten() {
                if let Some(name) = project.name {
                    println!("Project: {}", name);
                }
            }
        } else {
            panic!("Organization name not found");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_org_projects() {
        org_projects().await.unwrap();
    }
}
