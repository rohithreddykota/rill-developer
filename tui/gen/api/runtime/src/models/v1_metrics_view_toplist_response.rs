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
pub struct V1MetricsViewToplistResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Vec<models::V1MetricsViewColumn>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
}

impl V1MetricsViewToplistResponse {
    pub fn new() -> V1MetricsViewToplistResponse {
        V1MetricsViewToplistResponse {
            meta: None,
            data: None,
        }
    }
}

