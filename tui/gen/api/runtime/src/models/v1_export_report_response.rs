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
pub struct V1ExportReportResponse {
    #[serde(rename = "downloadUrlPath", skip_serializing_if = "Option::is_none")]
    pub download_url_path: Option<String>,
}

impl V1ExportReportResponse {
    pub fn new() -> V1ExportReportResponse {
        V1ExportReportResponse {
            download_url_path: None,
        }
    }
}

