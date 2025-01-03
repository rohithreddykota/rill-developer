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
pub struct StreamResultOfV1WatchLogsResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::V1WatchLogsResponse>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::RpcStatus>>,
}

impl StreamResultOfV1WatchLogsResponse {
    pub fn new() -> StreamResultOfV1WatchLogsResponse {
        StreamResultOfV1WatchLogsResponse {
            result: None,
            error: None,
        }
    }
}

