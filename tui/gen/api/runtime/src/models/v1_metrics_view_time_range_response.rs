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
pub struct V1MetricsViewTimeRangeResponse {
    #[serde(rename = "timeRangeSummary", skip_serializing_if = "Option::is_none")]
    pub time_range_summary: Option<Box<models::V1TimeRangeSummary>>,
}

impl V1MetricsViewTimeRangeResponse {
    pub fn new() -> V1MetricsViewTimeRangeResponse {
        V1MetricsViewTimeRangeResponse {
            time_range_summary: None,
        }
    }
}

