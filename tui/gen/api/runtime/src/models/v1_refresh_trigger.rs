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
pub struct V1RefreshTrigger {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::V1RefreshTriggerSpec>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
}

impl V1RefreshTrigger {
    pub fn new() -> V1RefreshTrigger {
        V1RefreshTrigger {
            spec: None,
            state: None,
        }
    }
}

