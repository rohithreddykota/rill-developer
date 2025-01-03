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
pub struct V1ProfileColumn {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "largestStringLength", skip_serializing_if = "Option::is_none")]
    pub largest_string_length: Option<i32>,
}

impl V1ProfileColumn {
    pub fn new() -> V1ProfileColumn {
        V1ProfileColumn {
            name: None,
            r#type: None,
            largest_string_length: None,
        }
    }
}

