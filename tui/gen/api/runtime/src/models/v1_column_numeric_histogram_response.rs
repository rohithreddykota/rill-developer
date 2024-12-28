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
pub struct V1ColumnNumericHistogramResponse {
    #[serde(rename = "numericSummary", skip_serializing_if = "Option::is_none")]
    pub numeric_summary: Option<Box<models::V1NumericSummary>>,
}

impl V1ColumnNumericHistogramResponse {
    pub fn new() -> V1ColumnNumericHistogramResponse {
        V1ColumnNumericHistogramResponse {
            numeric_summary: None,
        }
    }
}

