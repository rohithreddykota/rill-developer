/*
 * rill/admin/v1/ai.proto
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
pub struct V1UpdateServiceResponse {
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<models::V1Service>>,
}

impl V1UpdateServiceResponse {
    pub fn new() -> V1UpdateServiceResponse {
        V1UpdateServiceResponse {
            service: None,
        }
    }
}
