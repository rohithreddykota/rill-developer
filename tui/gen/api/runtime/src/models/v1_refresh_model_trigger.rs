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
pub struct V1RefreshModelTrigger {
    /// The model to refresh.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// If true, the current table and state will be dropped before refreshing. For non-incremental models, this is equivalent to a normal refresh.
    #[serde(rename = "full", skip_serializing_if = "Option::is_none")]
    pub full: Option<bool>,
    /// Keys of specific partitions to refresh.
    #[serde(rename = "partitions", skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<String>>,
    /// If true, it will refresh all partitions that errored on their last execution.
    #[serde(rename = "allErroredPartitions", skip_serializing_if = "Option::is_none")]
    pub all_errored_partitions: Option<bool>,
}

impl V1RefreshModelTrigger {
    pub fn new() -> V1RefreshModelTrigger {
        V1RefreshModelTrigger {
            model: None,
            full: None,
            partitions: None,
            all_errored_partitions: None,
        }
    }
}

