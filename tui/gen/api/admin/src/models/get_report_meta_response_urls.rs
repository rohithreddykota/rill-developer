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
pub struct GetReportMetaResponseUrls {
    #[serde(rename = "openUrl", skip_serializing_if = "Option::is_none")]
    pub open_url: Option<String>,
    #[serde(rename = "exportUrl", skip_serializing_if = "Option::is_none")]
    pub export_url: Option<String>,
    #[serde(rename = "editUrl", skip_serializing_if = "Option::is_none")]
    pub edit_url: Option<String>,
}

impl GetReportMetaResponseUrls {
    pub fn new() -> GetReportMetaResponseUrls {
        GetReportMetaResponseUrls {
            open_url: None,
            export_url: None,
            edit_url: None,
        }
    }
}

