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
pub struct V1Notifier {
    #[serde(rename = "connector", skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl V1Notifier {
    pub fn new() -> V1Notifier {
        V1Notifier {
            connector: None,
            properties: None,
        }
    }
}

