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
pub struct RequestMessageForRuntimeServiceUnpackExample {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl RequestMessageForRuntimeServiceUnpackExample {
    pub fn new() -> RequestMessageForRuntimeServiceUnpackExample {
        RequestMessageForRuntimeServiceUnpackExample {
            name: None,
            force: None,
        }
    }
}

