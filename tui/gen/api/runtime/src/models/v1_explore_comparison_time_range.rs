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
pub struct V1ExploreComparisonTimeRange {
    /// ISO 8601 duration string to use as an offset from the base time range.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// ISO 8601 duration string for the duration of the comparison time range. If not specified, it should fallback to the range of the base time range.
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

impl V1ExploreComparisonTimeRange {
    pub fn new() -> V1ExploreComparisonTimeRange {
        V1ExploreComparisonTimeRange {
            offset: None,
            range: None,
        }
    }
}

