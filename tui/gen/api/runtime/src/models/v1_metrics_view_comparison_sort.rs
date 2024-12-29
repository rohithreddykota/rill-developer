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
pub struct V1MetricsViewComparisonSort {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::V1MetricsViewComparisonSortType>,
    #[serde(rename = "sortType", skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<models::V1MetricsViewComparisonMeasureType>,
}

impl V1MetricsViewComparisonSort {
    pub fn new() -> V1MetricsViewComparisonSort {
        V1MetricsViewComparisonSort {
            name: None,
            desc: None,
            r#type: None,
            sort_type: None,
        }
    }
}
