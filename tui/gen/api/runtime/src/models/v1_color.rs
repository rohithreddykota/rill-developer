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
pub struct V1Color {
    #[serde(rename = "red", skip_serializing_if = "Option::is_none")]
    pub red: Option<f32>,
    #[serde(rename = "green", skip_serializing_if = "Option::is_none")]
    pub green: Option<f32>,
    #[serde(rename = "blue", skip_serializing_if = "Option::is_none")]
    pub blue: Option<f32>,
    #[serde(rename = "alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

impl V1Color {
    pub fn new() -> V1Color {
        V1Color {
            red: None,
            green: None,
            blue: None,
            alpha: None,
        }
    }
}
