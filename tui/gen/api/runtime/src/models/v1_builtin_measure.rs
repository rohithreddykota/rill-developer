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
pub enum V1BuiltinMeasure {
    #[serde(rename = "BUILTIN_MEASURE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BUILTIN_MEASURE_COUNT")]
    Count,
    #[serde(rename = "BUILTIN_MEASURE_COUNT_DISTINCT")]
    CountDistinct,

}

impl std::fmt::Display for V1BuiltinMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "BUILTIN_MEASURE_UNSPECIFIED"),
            Self::Count => write!(f, "BUILTIN_MEASURE_COUNT"),
            Self::CountDistinct => write!(f, "BUILTIN_MEASURE_COUNT_DISTINCT"),
        }
    }
}

impl Default for V1BuiltinMeasure {
    fn default() -> V1BuiltinMeasure {
        Self::Unspecified
    }
}

