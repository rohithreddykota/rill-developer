use crate::config::ClientConfig;
use crate::examples::load_data;
use admin::apis::admin_service_api as admin_api;

pub async fn org_projects() -> Result<(), Box<dyn std::error::Error>> {
    // load default configuration
    let config = ClientConfig::default();

    // load organizations
    let orgs = load_data("data/v1_list_organizations_response.json", || async {
        admin_api::admin_service_list_organizations(&config, None, None)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    })
    .await?;

    // iterate over organizations and projects
    for org in orgs.organizations.into_iter().flatten() {
        if let Some(name) = org.name {
            println!();
            println!("Organization: {}", name);
            let projects = load_data(
                &format!("data/v1_list_projects_for_organization_{}.json", name),
                || async {
                    admin_api::admin_service_list_projects_for_organization(
                        &config,
                        name.as_str(),
                        None,
                        None,
                    )
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                },
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
