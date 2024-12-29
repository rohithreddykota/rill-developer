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
pub struct V1AlertSpec {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<bool>,
    #[serde(rename = "refreshSchedule", skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<Box<models::V1Schedule>>,
    /// If true, will use the lowest watermark of its refs instead of the trigger time.
    #[serde(rename = "watermarkInherit", skip_serializing_if = "Option::is_none")]
    pub watermark_inherit: Option<bool>,
    #[serde(rename = "intervalsIsoDuration", skip_serializing_if = "Option::is_none")]
    pub intervals_iso_duration: Option<String>,
    #[serde(rename = "intervalsLimit", skip_serializing_if = "Option::is_none")]
    pub intervals_limit: Option<i32>,
    #[serde(rename = "intervalsCheckUnclosed", skip_serializing_if = "Option::is_none")]
    pub intervals_check_unclosed: Option<bool>,
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
    #[serde(rename = "queryName", skip_serializing_if = "Option::is_none")]
    pub query_name: Option<String>,
    #[serde(rename = "queryArgsJson", skip_serializing_if = "Option::is_none")]
    pub query_args_json: Option<String>,
    #[serde(rename = "resolver", skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    #[serde(rename = "resolverProperties", skip_serializing_if = "Option::is_none")]
    pub resolver_properties: Option<serde_json::Value>,
    #[serde(rename = "queryForUserId", skip_serializing_if = "Option::is_none")]
    pub query_for_user_id: Option<String>,
    #[serde(rename = "queryForUserEmail", skip_serializing_if = "Option::is_none")]
    pub query_for_user_email: Option<String>,
    #[serde(rename = "queryForAttributes", skip_serializing_if = "Option::is_none")]
    pub query_for_attributes: Option<serde_json::Value>,
    #[serde(rename = "notifyOnRecover", skip_serializing_if = "Option::is_none")]
    pub notify_on_recover: Option<bool>,
    #[serde(rename = "notifyOnFail", skip_serializing_if = "Option::is_none")]
    pub notify_on_fail: Option<bool>,
    #[serde(rename = "notifyOnError", skip_serializing_if = "Option::is_none")]
    pub notify_on_error: Option<bool>,
    #[serde(rename = "renotify", skip_serializing_if = "Option::is_none")]
    pub renotify: Option<bool>,
    #[serde(rename = "renotifyAfterSeconds", skip_serializing_if = "Option::is_none")]
    pub renotify_after_seconds: Option<i64>,
    #[serde(rename = "notifiers", skip_serializing_if = "Option::is_none")]
    pub notifiers: Option<Vec<models::V1Notifier>>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
}

impl V1AlertSpec {
    pub fn new() -> V1AlertSpec {
        V1AlertSpec {
            display_name: None,
            trigger: None,
            refresh_schedule: None,
            watermark_inherit: None,
            intervals_iso_duration: None,
            intervals_limit: None,
            intervals_check_unclosed: None,
            timeout_seconds: None,
            query_name: None,
            query_args_json: None,
            resolver: None,
            resolver_properties: None,
            query_for_user_id: None,
            query_for_user_email: None,
            query_for_attributes: None,
            notify_on_recover: None,
            notify_on_fail: None,
            notify_on_error: None,
            renotify: None,
            renotify_after_seconds: None,
            notifiers: None,
            annotations: None,
        }
    }
}

