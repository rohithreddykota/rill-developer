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
pub struct V1MetricsViewTimeSeriesRequest {
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "metricsViewName", skip_serializing_if = "Option::is_none")]
    pub metrics_view_name: Option<String>,
    #[serde(rename = "measureNames", skip_serializing_if = "Option::is_none")]
    pub measure_names: Option<Vec<String>>,
    #[serde(rename = "timeStart", skip_serializing_if = "Option::is_none")]
    pub time_start: Option<String>,
    #[serde(rename = "timeEnd", skip_serializing_if = "Option::is_none")]
    pub time_end: Option<String>,
    #[serde(rename = "timeGranularity", skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<models::V1TimeGrain>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<models::V1Expression>>,
    /// Optional. If both where and where_sql are set, both will be applied with an AND between them.
    #[serde(rename = "whereSql", skip_serializing_if = "Option::is_none")]
    pub where_sql: Option<String>,
    #[serde(rename = "having", skip_serializing_if = "Option::is_none")]
    pub having: Option<Box<models::V1Expression>>,
    /// Optional. If both having and having_sql are set, both will be applied with an AND between them.
    #[serde(rename = "havingSql", skip_serializing_if = "Option::is_none")]
    pub having_sql: Option<String>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<models::V1MetricsViewFilter>>,
}

impl V1MetricsViewTimeSeriesRequest {
    pub fn new() -> V1MetricsViewTimeSeriesRequest {
        V1MetricsViewTimeSeriesRequest {
            instance_id: None,
            metrics_view_name: None,
            measure_names: None,
            time_start: None,
            time_end: None,
            time_granularity: None,
            r#where: None,
            where_sql: None,
            having: None,
            having_sql: None,
            time_zone: None,
            priority: None,
            filter: None,
        }
    }
}

