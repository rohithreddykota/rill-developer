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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1AssertionResult {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::V1AssertionStatus>,
    #[serde(rename = "failRow", skip_serializing_if = "Option::is_none")]
    pub fail_row: Option<serde_json::Value>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl V1AssertionResult {
    pub fn new() -> V1AssertionResult {
        V1AssertionResult {
            status: None,
            fail_row: None,
            error_message: None,
        }
    }
}

