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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum V1MetricsViewComparisonSortType {
    #[serde(rename = "METRICS_VIEW_COMPARISON_SORT_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "METRICS_VIEW_COMPARISON_SORT_TYPE_BASE_VALUE")]
    BaseValue,
    #[serde(rename = "METRICS_VIEW_COMPARISON_SORT_TYPE_COMPARISON_VALUE")]
    ComparisonValue,
    #[serde(rename = "METRICS_VIEW_COMPARISON_SORT_TYPE_ABS_DELTA")]
    AbsDelta,
    #[serde(rename = "METRICS_VIEW_COMPARISON_SORT_TYPE_REL_DELTA")]
    RelDelta,

}

impl std::fmt::Display for V1MetricsViewComparisonSortType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "METRICS_VIEW_COMPARISON_SORT_TYPE_UNSPECIFIED"),
            Self::BaseValue => write!(f, "METRICS_VIEW_COMPARISON_SORT_TYPE_BASE_VALUE"),
            Self::ComparisonValue => write!(f, "METRICS_VIEW_COMPARISON_SORT_TYPE_COMPARISON_VALUE"),
            Self::AbsDelta => write!(f, "METRICS_VIEW_COMPARISON_SORT_TYPE_ABS_DELTA"),
            Self::RelDelta => write!(f, "METRICS_VIEW_COMPARISON_SORT_TYPE_REL_DELTA"),
        }
    }
}

impl Default for V1MetricsViewComparisonSortType {
    fn default() -> V1MetricsViewComparisonSortType {
        Self::Unspecified
    }
}
