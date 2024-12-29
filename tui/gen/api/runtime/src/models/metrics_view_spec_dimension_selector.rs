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
pub struct MetricsViewSpecDimensionSelector {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "timeGrain", skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<models::V1TimeGrain>,
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

impl MetricsViewSpecDimensionSelector {
    pub fn new() -> MetricsViewSpecDimensionSelector {
        MetricsViewSpecDimensionSelector {
            name: None,
            time_grain: None,
            desc: None,
        }
    }
}

