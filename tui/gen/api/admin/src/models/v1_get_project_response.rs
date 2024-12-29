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
pub struct V1GetProjectResponse {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::V1Project>>,
    #[serde(rename = "prodDeployment", skip_serializing_if = "Option::is_none")]
    pub prod_deployment: Option<Box<models::V1Deployment>>,
    #[serde(rename = "jwt", skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,
    #[serde(rename = "projectPermissions", skip_serializing_if = "Option::is_none")]
    pub project_permissions: Option<Box<models::V1ProjectPermissions>>,
}

impl V1GetProjectResponse {
    pub fn new() -> V1GetProjectResponse {
        V1GetProjectResponse {
            project: None,
            prod_deployment: None,
            jwt: None,
            project_permissions: None,
        }
    }
}
