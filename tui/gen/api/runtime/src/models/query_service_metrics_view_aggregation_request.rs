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
pub struct QueryServiceMetricsViewAggregationRequest {
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<models::V1MetricsViewAggregationDimension>>,
    #[serde(rename = "measures", skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<models::V1MetricsViewAggregationMeasure>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<models::V1MetricsViewAggregationSort>>,
    #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
    pub time_range: Option<Box<models::V1TimeRange>>,
    #[serde(rename = "comparisonTimeRange", skip_serializing_if = "Option::is_none")]
    pub comparison_time_range: Option<Box<models::V1TimeRange>>,
    #[serde(rename = "timeStart", skip_serializing_if = "Option::is_none")]
    pub time_start: Option<String>,
    #[serde(rename = "timeEnd", skip_serializing_if = "Option::is_none")]
    pub time_end: Option<String>,
    #[serde(rename = "pivotOn", skip_serializing_if = "Option::is_none")]
    pub pivot_on: Option<Vec<String>>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<models::V1MetricsViewComparisonMeasureAlias>>,
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
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<models::V1MetricsViewFilter>>,
    #[serde(rename = "exact", skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl QueryServiceMetricsViewAggregationRequest {
    pub fn new() -> QueryServiceMetricsViewAggregationRequest {
        QueryServiceMetricsViewAggregationRequest {
            dimensions: None,
            measures: None,
            sort: None,
            time_range: None,
            comparison_time_range: None,
            time_start: None,
            time_end: None,
            pivot_on: None,
            aliases: None,
            r#where: None,
            where_sql: None,
            having: None,
            having_sql: None,
            limit: None,
            offset: None,
            priority: None,
            filter: None,
            exact: None,
        }
    }
}

