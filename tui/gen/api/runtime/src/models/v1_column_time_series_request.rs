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
pub struct V1ColumnTimeSeriesRequest {
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
    #[serde(rename = "measures", skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<models::ColumnTimeSeriesRequestBasicMeasure>>,
    #[serde(rename = "timestampColumnName", skip_serializing_if = "Option::is_none")]
    pub timestamp_column_name: Option<String>,
    #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
    pub time_range: Option<Box<models::V1TimeSeriesTimeRange>>,
    #[serde(rename = "pixels", skip_serializing_if = "Option::is_none")]
    pub pixels: Option<i32>,
    #[serde(rename = "sampleSize", skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i32>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl V1ColumnTimeSeriesRequest {
    pub fn new() -> V1ColumnTimeSeriesRequest {
        V1ColumnTimeSeriesRequest {
            instance_id: None,
            connector: None,
            database: None,
            database_schema: None,
            table_name: None,
            measures: None,
            timestamp_column_name: None,
            time_range: None,
            pixels: None,
            sample_size: None,
            priority: None,
            time_zone: None,
        }
    }
}

