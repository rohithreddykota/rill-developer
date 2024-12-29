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
pub struct V1UpdateProjectVariablesResponse {
    /// Variables that were created or updated by the request.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<models::V1ProjectVariable>>,
}

impl V1UpdateProjectVariablesResponse {
    pub fn new() -> V1UpdateProjectVariablesResponse {
        V1UpdateProjectVariablesResponse {
            variables: None,
        }
    }
}
