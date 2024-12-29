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
pub struct AdminServiceCreateAssetRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "cacheable", skip_serializing_if = "Option::is_none")]
    pub cacheable: Option<bool>,
    #[serde(rename = "estimatedSizeBytes", skip_serializing_if = "Option::is_none")]
    pub estimated_size_bytes: Option<String>,
}

impl AdminServiceCreateAssetRequest {
    pub fn new() -> AdminServiceCreateAssetRequest {
        AdminServiceCreateAssetRequest {
            r#type: None,
            name: None,
            extension: None,
            cacheable: None,
            estimated_size_bytes: None,
        }
    }
}

