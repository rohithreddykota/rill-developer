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
pub struct V1GetResourceResponse {
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<models::V1Resource>>,
}

impl V1GetResourceResponse {
    pub fn new() -> V1GetResourceResponse {
        V1GetResourceResponse {
            resource: None,
        }
    }
}
