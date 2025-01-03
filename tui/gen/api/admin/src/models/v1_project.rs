/*
 * rill/admin/v1/ai.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: version not set
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1Project {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "orgName", skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "createdByUserId", skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[serde(rename = "provisioner", skip_serializing_if = "Option::is_none")]
    pub provisioner: Option<String>,
    #[serde(rename = "githubUrl", skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    #[serde(rename = "subpath", skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
    #[serde(rename = "prodBranch", skip_serializing_if = "Option::is_none")]
    pub prod_branch: Option<String>,
    #[serde(rename = "archiveAssetId", skip_serializing_if = "Option::is_none")]
    pub archive_asset_id: Option<String>,
    #[serde(rename = "prodOlapDriver", skip_serializing_if = "Option::is_none")]
    pub prod_olap_driver: Option<String>,
    #[serde(rename = "prodOlapDsn", skip_serializing_if = "Option::is_none")]
    pub prod_olap_dsn: Option<String>,
    #[serde(rename = "prodSlots", skip_serializing_if = "Option::is_none")]
    pub prod_slots: Option<String>,
    #[serde(rename = "prodDeploymentId", skip_serializing_if = "Option::is_none")]
    pub prod_deployment_id: Option<String>,
    /// Note: Does NOT incorporate the parent org's custom domain.
    #[serde(rename = "frontendUrl", skip_serializing_if = "Option::is_none")]
    pub frontend_url: Option<String>,
    #[serde(rename = "prodTtlSeconds", skip_serializing_if = "Option::is_none")]
    pub prod_ttl_seconds: Option<String>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "prodVersion", skip_serializing_if = "Option::is_none")]
    pub prod_version: Option<String>,
    #[serde(rename = "createdOn", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "updatedOn", skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,
}

impl V1Project {
    pub fn new() -> V1Project {
        V1Project {
            id: None,
            name: None,
            org_id: None,
            org_name: None,
            description: None,
            public: None,
            created_by_user_id: None,
            provisioner: None,
            github_url: None,
            subpath: None,
            prod_branch: None,
            archive_asset_id: None,
            prod_olap_driver: None,
            prod_olap_dsn: None,
            prod_slots: None,
            prod_deployment_id: None,
            frontend_url: None,
            prod_ttl_seconds: None,
            annotations: None,
            prod_version: None,
            created_on: None,
            updated_on: None,
        }
    }
}

