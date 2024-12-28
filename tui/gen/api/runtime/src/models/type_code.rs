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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeCode {
    #[serde(rename = "CODE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "CODE_BOOL")]
    Bool,
    #[serde(rename = "CODE_INT8")]
    Int8,
    #[serde(rename = "CODE_INT16")]
    Int16,
    #[serde(rename = "CODE_INT32")]
    Int32,
    #[serde(rename = "CODE_INT64")]
    Int64,
    #[serde(rename = "CODE_INT128")]
    Int128,
    #[serde(rename = "CODE_INT256")]
    Int256,
    #[serde(rename = "CODE_UINT8")]
    Uint8,
    #[serde(rename = "CODE_UINT16")]
    Uint16,
    #[serde(rename = "CODE_UINT32")]
    Uint32,
    #[serde(rename = "CODE_UINT64")]
    Uint64,
    #[serde(rename = "CODE_UINT128")]
    Uint128,
    #[serde(rename = "CODE_UINT256")]
    Uint256,
    #[serde(rename = "CODE_FLOAT32")]
    Float32,
    #[serde(rename = "CODE_FLOAT64")]
    Float64,
    #[serde(rename = "CODE_TIMESTAMP")]
    Timestamp,
    #[serde(rename = "CODE_INTERVAL")]
    Interval,
    #[serde(rename = "CODE_DATE")]
    Date,
    #[serde(rename = "CODE_TIME")]
    Time,
    #[serde(rename = "CODE_STRING")]
    String,
    #[serde(rename = "CODE_BYTES")]
    Bytes,
    #[serde(rename = "CODE_ARRAY")]
    Array,
    #[serde(rename = "CODE_STRUCT")]
    Struct,
    #[serde(rename = "CODE_MAP")]
    Map,
    #[serde(rename = "CODE_DECIMAL")]
    Decimal,
    #[serde(rename = "CODE_JSON")]
    Json,
    #[serde(rename = "CODE_UUID")]
    Uuid,

}

impl std::fmt::Display for TypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "CODE_UNSPECIFIED"),
            Self::Bool => write!(f, "CODE_BOOL"),
            Self::Int8 => write!(f, "CODE_INT8"),
            Self::Int16 => write!(f, "CODE_INT16"),
            Self::Int32 => write!(f, "CODE_INT32"),
            Self::Int64 => write!(f, "CODE_INT64"),
            Self::Int128 => write!(f, "CODE_INT128"),
            Self::Int256 => write!(f, "CODE_INT256"),
            Self::Uint8 => write!(f, "CODE_UINT8"),
            Self::Uint16 => write!(f, "CODE_UINT16"),
            Self::Uint32 => write!(f, "CODE_UINT32"),
            Self::Uint64 => write!(f, "CODE_UINT64"),
            Self::Uint128 => write!(f, "CODE_UINT128"),
            Self::Uint256 => write!(f, "CODE_UINT256"),
            Self::Float32 => write!(f, "CODE_FLOAT32"),
            Self::Float64 => write!(f, "CODE_FLOAT64"),
            Self::Timestamp => write!(f, "CODE_TIMESTAMP"),
            Self::Interval => write!(f, "CODE_INTERVAL"),
            Self::Date => write!(f, "CODE_DATE"),
            Self::Time => write!(f, "CODE_TIME"),
            Self::String => write!(f, "CODE_STRING"),
            Self::Bytes => write!(f, "CODE_BYTES"),
            Self::Array => write!(f, "CODE_ARRAY"),
            Self::Struct => write!(f, "CODE_STRUCT"),
            Self::Map => write!(f, "CODE_MAP"),
            Self::Decimal => write!(f, "CODE_DECIMAL"),
            Self::Json => write!(f, "CODE_JSON"),
            Self::Uuid => write!(f, "CODE_UUID"),
        }
    }
}

impl Default for TypeCode {
    fn default() -> TypeCode {
        Self::Unspecified
    }
}

