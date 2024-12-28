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
pub struct V1ConnectorState {
    #[serde(rename = "specHash", skip_serializing_if = "Option::is_none")]
    pub spec_hash: Option<String>,
}

impl V1ConnectorState {
    pub fn new() -> V1ConnectorState {
        V1ConnectorState {
            spec_hash: None,
        }
    }
}

