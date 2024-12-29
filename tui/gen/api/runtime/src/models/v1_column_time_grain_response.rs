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
pub struct V1ColumnTimeGrainResponse {
    #[serde(rename = "timeGrain", skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<models::V1TimeGrain>,
}

impl V1ColumnTimeGrainResponse {
    pub fn new() -> V1ColumnTimeGrainResponse {
        V1ColumnTimeGrainResponse {
            time_grain: None,
        }
    }
}

