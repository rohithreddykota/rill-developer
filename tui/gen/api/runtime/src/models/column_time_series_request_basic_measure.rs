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
pub struct ColumnTimeSeriesRequestBasicMeasure {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "sqlName", skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,
}

impl ColumnTimeSeriesRequestBasicMeasure {
    pub fn new() -> ColumnTimeSeriesRequestBasicMeasure {
        ColumnTimeSeriesRequestBasicMeasure {
            id: None,
            expression: None,
            sql_name: None,
        }
    }
}

