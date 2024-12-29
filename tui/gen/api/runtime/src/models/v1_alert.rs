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
pub struct V1Alert {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::V1AlertSpec>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<models::V1AlertState>>,
}

impl V1Alert {
    pub fn new() -> V1Alert {
        V1Alert {
            spec: None,
            state: None,
        }
    }
}

