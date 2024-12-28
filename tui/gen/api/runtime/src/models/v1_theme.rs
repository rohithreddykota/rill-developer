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
pub struct V1Theme {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::V1ThemeSpec>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
}

impl V1Theme {
    pub fn new() -> V1Theme {
        V1Theme {
            spec: None,
            state: None,
        }
    }
}

