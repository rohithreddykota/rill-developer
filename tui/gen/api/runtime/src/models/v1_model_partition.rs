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
pub struct V1ModelPartition {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "watermark", skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,
    #[serde(rename = "executedOn", skip_serializing_if = "Option::is_none")]
    pub executed_on: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "elapsedMs", skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<i64>,
}

impl V1ModelPartition {
    pub fn new() -> V1ModelPartition {
        V1ModelPartition {
            key: None,
            data: None,
            watermark: None,
            executed_on: None,
            error: None,
            elapsed_ms: None,
        }
    }
}
