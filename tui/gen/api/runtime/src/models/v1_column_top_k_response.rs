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
pub struct V1ColumnTopKResponse {
    #[serde(rename = "categoricalSummary", skip_serializing_if = "Option::is_none")]
    pub categorical_summary: Option<Box<models::V1CategoricalSummary>>,
}

impl V1ColumnTopKResponse {
    pub fn new() -> V1ColumnTopKResponse {
        V1ColumnTopKResponse {
            categorical_summary: None,
        }
    }
}
