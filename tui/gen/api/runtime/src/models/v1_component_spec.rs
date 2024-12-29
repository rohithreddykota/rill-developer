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
pub struct V1ComponentSpec {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resolver", skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    #[serde(rename = "resolverProperties", skip_serializing_if = "Option::is_none")]
    pub resolver_properties: Option<serde_json::Value>,
    #[serde(rename = "renderer", skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
    #[serde(rename = "rendererProperties", skip_serializing_if = "Option::is_none")]
    pub renderer_properties: Option<serde_json::Value>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<models::V1ComponentVariable>>,
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<Box<models::V1ComponentVariable>>,
    /// Templated string that should evaluate to a boolean.
    #[serde(rename = "show", skip_serializing_if = "Option::is_none")]
    pub show: Option<String>,
    #[serde(rename = "definedInCanvas", skip_serializing_if = "Option::is_none")]
    pub defined_in_canvas: Option<bool>,
}

impl V1ComponentSpec {
    pub fn new() -> V1ComponentSpec {
        V1ComponentSpec {
            display_name: None,
            description: None,
            resolver: None,
            resolver_properties: None,
            renderer: None,
            renderer_properties: None,
            input: None,
            output: None,
            show: None,
            defined_in_canvas: None,
        }
    }
}

