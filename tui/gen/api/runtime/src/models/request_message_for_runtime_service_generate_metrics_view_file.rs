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
pub struct RequestMessageForRuntimeServiceGenerateMetricsViewFile {
    #[serde(rename = "connector", skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "databaseSchema", skip_serializing_if = "Option::is_none")]
    pub database_schema: Option<String>,
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "useAi", skip_serializing_if = "Option::is_none")]
    pub use_ai: Option<bool>,
}

impl RequestMessageForRuntimeServiceGenerateMetricsViewFile {
    pub fn new() -> RequestMessageForRuntimeServiceGenerateMetricsViewFile {
        RequestMessageForRuntimeServiceGenerateMetricsViewFile {
            connector: None,
            database: None,
            database_schema: None,
            table: None,
            path: None,
            use_ai: None,
        }
    }
}

