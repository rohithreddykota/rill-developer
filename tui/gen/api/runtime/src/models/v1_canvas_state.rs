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
pub struct V1CanvasState {
    #[serde(rename = "validSpec", skip_serializing_if = "Option::is_none")]
    pub valid_spec: Option<Box<models::V1CanvasSpec>>,
}

impl V1CanvasState {
    pub fn new() -> V1CanvasState {
        V1CanvasState {
            valid_spec: None,
        }
    }
}

