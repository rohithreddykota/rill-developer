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
pub struct V1ColumnNumericHistogramRequest {
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "connector", skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "databaseSchema", skip_serializing_if = "Option::is_none")]
    pub database_schema: Option<String>,
    #[serde(rename = "tableName", skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "columnName", skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "histogramMethod", skip_serializing_if = "Option::is_none")]
    pub histogram_method: Option<models::V1HistogramMethod>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl V1ColumnNumericHistogramRequest {
    pub fn new() -> V1ColumnNumericHistogramRequest {
        V1ColumnNumericHistogramRequest {
            instance_id: None,
            connector: None,
            database: None,
            database_schema: None,
            table_name: None,
            column_name: None,
            histogram_method: None,
            priority: None,
        }
    }
}

