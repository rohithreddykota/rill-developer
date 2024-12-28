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
pub struct V1TableInfo {
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "databaseSchema", skip_serializing_if = "Option::is_none")]
    pub database_schema: Option<String>,
    #[serde(rename = "isDefaultDatabase", skip_serializing_if = "Option::is_none")]
    pub is_default_database: Option<bool>,
    #[serde(rename = "isDefaultDatabaseSchema", skip_serializing_if = "Option::is_none")]
    pub is_default_database_schema: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "hasUnsupportedDataTypes", skip_serializing_if = "Option::is_none")]
    pub has_unsupported_data_types: Option<bool>,
}

impl V1TableInfo {
    pub fn new() -> V1TableInfo {
        V1TableInfo {
            database: None,
            database_schema: None,
            is_default_database: None,
            is_default_database_schema: None,
            name: None,
            has_unsupported_data_types: None,
        }
    }
}

