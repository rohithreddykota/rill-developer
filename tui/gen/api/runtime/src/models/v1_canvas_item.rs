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
pub struct V1CanvasItem {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "definedInCanvas", skip_serializing_if = "Option::is_none")]
    pub defined_in_canvas: Option<bool>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

impl V1CanvasItem {
    pub fn new() -> V1CanvasItem {
        V1CanvasItem {
            component: None,
            defined_in_canvas: None,
            x: None,
            y: None,
            width: None,
            height: None,
        }
    }
}

