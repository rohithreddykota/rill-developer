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
pub struct QueryServiceMetricsViewSearchRequest {
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
    pub time_range: Option<Box<models::V1TimeRange>>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<models::V1Expression>>,
    #[serde(rename = "having", skip_serializing_if = "Option::is_none")]
    pub having: Option<Box<models::V1Expression>>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl QueryServiceMetricsViewSearchRequest {
    pub fn new() -> QueryServiceMetricsViewSearchRequest {
        QueryServiceMetricsViewSearchRequest {
            dimensions: None,
            search: None,
            time_range: None,
            r#where: None,
            having: None,
            limit: None,
            priority: None,
        }
    }
}

