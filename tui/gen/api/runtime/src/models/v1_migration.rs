/*
 * rill/runtime/v1/colors.proto
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
pub struct V1Migration {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::V1MigrationSpec>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<models::V1MigrationState>>,
}

impl V1Migration {
    pub fn new() -> V1Migration {
        V1Migration {
            spec: None,
            state: None,
        }
    }
}

