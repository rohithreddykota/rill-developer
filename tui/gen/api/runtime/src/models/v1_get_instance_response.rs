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
pub struct V1GetInstanceResponse {
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<models::V1Instance>>,
}

impl V1GetInstanceResponse {
    pub fn new() -> V1GetInstanceResponse {
        V1GetInstanceResponse {
            instance: None,
        }
    }
}

