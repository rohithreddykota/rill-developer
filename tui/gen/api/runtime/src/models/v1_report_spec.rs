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
pub struct V1ReportSpec {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<bool>,
    #[serde(rename = "refreshSchedule", skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<Box<models::V1Schedule>>,
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
    #[serde(rename = "queryName", skip_serializing_if = "Option::is_none")]
    pub query_name: Option<String>,
    #[serde(rename = "queryArgsJson", skip_serializing_if = "Option::is_none")]
    pub query_args_json: Option<String>,
    #[serde(rename = "exportLimit", skip_serializing_if = "Option::is_none")]
    pub export_limit: Option<String>,
    #[serde(rename = "exportFormat", skip_serializing_if = "Option::is_none")]
    pub export_format: Option<models::V1ExportFormat>,
    #[serde(rename = "notifiers", skip_serializing_if = "Option::is_none")]
    pub notifiers: Option<Vec<models::V1Notifier>>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// If true, will use the lowest watermark of its refs instead of the trigger time.
    #[serde(rename = "watermarkInherit", skip_serializing_if = "Option::is_none")]
    pub watermark_inherit: Option<bool>,
    #[serde(rename = "intervalsIsoDuration", skip_serializing_if = "Option::is_none")]
    pub intervals_iso_duration: Option<String>,
    #[serde(rename = "intervalsLimit", skip_serializing_if = "Option::is_none")]
    pub intervals_limit: Option<i32>,
    #[serde(rename = "intervalsCheckUnclosed", skip_serializing_if = "Option::is_none")]
    pub intervals_check_unclosed: Option<bool>,
}

impl V1ReportSpec {
    pub fn new() -> V1ReportSpec {
        V1ReportSpec {
            display_name: None,
            trigger: None,
            refresh_schedule: None,
            timeout_seconds: None,
            query_name: None,
            query_args_json: None,
            export_limit: None,
            export_format: None,
            notifiers: None,
            annotations: None,
            watermark_inherit: None,
            intervals_iso_duration: None,
            intervals_limit: None,
            intervals_check_unclosed: None,
        }
    }
}
