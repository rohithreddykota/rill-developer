/*
 * rill/admin/v1/ai.proto
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
pub enum V1ExportFormat {
    #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "EXPORT_FORMAT_CSV")]
    Csv,
    #[serde(rename = "EXPORT_FORMAT_XLSX")]
    Xlsx,
    #[serde(rename = "EXPORT_FORMAT_PARQUET")]
    Parquet,

}

impl std::fmt::Display for V1ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "EXPORT_FORMAT_UNSPECIFIED"),
            Self::Csv => write!(f, "EXPORT_FORMAT_CSV"),
            Self::Xlsx => write!(f, "EXPORT_FORMAT_XLSX"),
            Self::Parquet => write!(f, "EXPORT_FORMAT_PARQUET"),
        }
    }
}

impl Default for V1ExportFormat {
    fn default() -> V1ExportFormat {
        Self::Unspecified
    }
}

