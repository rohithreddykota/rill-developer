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
pub struct V1GetFileResponse {
    #[serde(rename = "blob", skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(rename = "updatedOn", skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,
}

impl V1GetFileResponse {
    pub fn new() -> V1GetFileResponse {
        V1GetFileResponse {
            blob: None,
            updated_on: None,
        }
    }
}

