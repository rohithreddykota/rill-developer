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
pub struct V1MetricsViewSearchResponse {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::MetricsViewSearchResponseSearchResult>>,
}

impl V1MetricsViewSearchResponse {
    pub fn new() -> V1MetricsViewSearchResponse {
        V1MetricsViewSearchResponse {
            results: None,
        }
    }
}
