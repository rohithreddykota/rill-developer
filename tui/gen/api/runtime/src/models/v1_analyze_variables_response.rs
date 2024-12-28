/*
 * rill/runtime/v1/colors.proto
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
pub struct V1AnalyzeVariablesResponse {
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<models::V1AnalyzedVariable>>,
}

impl V1AnalyzeVariablesResponse {
    pub fn new() -> V1AnalyzeVariablesResponse {
        V1AnalyzeVariablesResponse {
            variables: None,
        }
    }
}

