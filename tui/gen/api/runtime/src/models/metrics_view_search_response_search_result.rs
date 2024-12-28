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
pub struct MetricsViewSearchResponseSearchResult {
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl MetricsViewSearchResponseSearchResult {
    pub fn new() -> MetricsViewSearchResponseSearchResult {
        MetricsViewSearchResponseSearchResult {
            dimension: None,
            value: None,
        }
    }
}

