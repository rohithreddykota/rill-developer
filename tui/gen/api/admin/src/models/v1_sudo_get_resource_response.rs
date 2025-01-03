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
pub struct V1SudoGetResourceResponse {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::V1User>>,
    #[serde(rename = "org", skip_serializing_if = "Option::is_none")]
    pub org: Option<Box<models::V1Organization>>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::V1Project>>,
    #[serde(rename = "deployment", skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Box<models::V1Deployment>>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<models::V1Deployment>>,
}

impl V1SudoGetResourceResponse {
    pub fn new() -> V1SudoGetResourceResponse {
        V1SudoGetResourceResponse {
            user: None,
            org: None,
            project: None,
            deployment: None,
            instance: None,
        }
    }
}

