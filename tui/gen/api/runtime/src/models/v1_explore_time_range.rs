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
pub struct V1ExploreTimeRange {
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(rename = "comparisonTimeRanges", skip_serializing_if = "Option::is_none")]
    pub comparison_time_ranges: Option<Vec<models::V1ExploreComparisonTimeRange>>,
}

impl V1ExploreTimeRange {
    pub fn new() -> V1ExploreTimeRange {
        V1ExploreTimeRange {
            range: None,
            comparison_time_ranges: None,
        }
    }
}

