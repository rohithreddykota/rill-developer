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
pub struct V1ColumnRollupIntervalResponse {
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<models::V1TimeGrain>,
}

impl V1ColumnRollupIntervalResponse {
    pub fn new() -> V1ColumnRollupIntervalResponse {
        V1ColumnRollupIntervalResponse {
            start: None,
            end: None,
            interval: None,
        }
    }
}

