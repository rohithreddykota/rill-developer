/*
 * rill/runtime/v1/api.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: version not set
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// V1AnalyzedConnector : AnalyzedConnector contains information about a connector that is referenced in the project files.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1AnalyzedConnector {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<Box<models::V1ConnectorDriver>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "presetConfig", skip_serializing_if = "Option::is_none")]
    pub preset_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "projectConfig", skip_serializing_if = "Option::is_none")]
    pub project_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "envConfig", skip_serializing_if = "Option::is_none")]
    pub env_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "provision", skip_serializing_if = "Option::is_none")]
    pub provision: Option<bool>,
    #[serde(rename = "provisionArgs", skip_serializing_if = "Option::is_none")]
    pub provision_args: Option<serde_json::Value>,
    #[serde(rename = "hasAnonymousAccess", skip_serializing_if = "Option::is_none")]
    pub has_anonymous_access: Option<bool>,
    #[serde(rename = "usedBy", skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<models::V1ResourceName>>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl V1AnalyzedConnector {
    /// AnalyzedConnector contains information about a connector that is referenced in the project files.
    pub fn new() -> V1AnalyzedConnector {
        V1AnalyzedConnector {
            name: None,
            driver: None,
            config: None,
            preset_config: None,
            project_config: None,
            env_config: None,
            provision: None,
            provision_args: None,
            has_anonymous_access: None,
            used_by: None,
            error_message: None,
        }
    }
}

