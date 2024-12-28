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
pub struct V1ColumnTimeRangeResponse {
    #[serde(rename = "timeRangeSummary", skip_serializing_if = "Option::is_none")]
    pub time_range_summary: Option<Box<models::V1TimeRangeSummary>>,
}

impl V1ColumnTimeRangeResponse {
    pub fn new() -> V1ColumnTimeRangeResponse {
        V1ColumnTimeRangeResponse {
            time_range_summary: None,
        }
    }
}

