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
pub enum V1TimeGrain {
    #[serde(rename = "TIME_GRAIN_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "TIME_GRAIN_MILLISECOND")]
    Millisecond,
    #[serde(rename = "TIME_GRAIN_SECOND")]
    Second,
    #[serde(rename = "TIME_GRAIN_MINUTE")]
    Minute,
    #[serde(rename = "TIME_GRAIN_HOUR")]
    Hour,
    #[serde(rename = "TIME_GRAIN_DAY")]
    Day,
    #[serde(rename = "TIME_GRAIN_WEEK")]
    Week,
    #[serde(rename = "TIME_GRAIN_MONTH")]
    Month,
    #[serde(rename = "TIME_GRAIN_QUARTER")]
    Quarter,
    #[serde(rename = "TIME_GRAIN_YEAR")]
    Year,

}

impl std::fmt::Display for V1TimeGrain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "TIME_GRAIN_UNSPECIFIED"),
            Self::Millisecond => write!(f, "TIME_GRAIN_MILLISECOND"),
            Self::Second => write!(f, "TIME_GRAIN_SECOND"),
            Self::Minute => write!(f, "TIME_GRAIN_MINUTE"),
            Self::Hour => write!(f, "TIME_GRAIN_HOUR"),
            Self::Day => write!(f, "TIME_GRAIN_DAY"),
            Self::Week => write!(f, "TIME_GRAIN_WEEK"),
            Self::Month => write!(f, "TIME_GRAIN_MONTH"),
            Self::Quarter => write!(f, "TIME_GRAIN_QUARTER"),
            Self::Year => write!(f, "TIME_GRAIN_YEAR"),
        }
    }
}

impl Default for V1TimeGrain {
    fn default() -> V1TimeGrain {
        Self::Unspecified
    }
}

