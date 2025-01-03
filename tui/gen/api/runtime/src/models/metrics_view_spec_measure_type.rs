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
pub enum MetricsViewSpecMeasureType {
    #[serde(rename = "MEASURE_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "MEASURE_TYPE_SIMPLE")]
    Simple,
    #[serde(rename = "MEASURE_TYPE_DERIVED")]
    Derived,
    #[serde(rename = "MEASURE_TYPE_TIME_COMPARISON")]
    TimeComparison,

}

impl std::fmt::Display for MetricsViewSpecMeasureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "MEASURE_TYPE_UNSPECIFIED"),
            Self::Simple => write!(f, "MEASURE_TYPE_SIMPLE"),
            Self::Derived => write!(f, "MEASURE_TYPE_DERIVED"),
            Self::TimeComparison => write!(f, "MEASURE_TYPE_TIME_COMPARISON"),
        }
    }
}

impl Default for MetricsViewSpecMeasureType {
    fn default() -> MetricsViewSpecMeasureType {
        Self::Unspecified
    }
}

