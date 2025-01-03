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
pub struct V1AlertExecution {
    #[serde(rename = "adhoc", skip_serializing_if = "Option::is_none")]
    pub adhoc: Option<bool>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::V1AssertionResult>>,
    #[serde(rename = "sentNotifications", skip_serializing_if = "Option::is_none")]
    pub sent_notifications: Option<bool>,
    #[serde(rename = "executionTime", skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
    #[serde(rename = "startedOn", skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    #[serde(rename = "finishedOn", skip_serializing_if = "Option::is_none")]
    pub finished_on: Option<String>,
    #[serde(rename = "suppressedSince", skip_serializing_if = "Option::is_none")]
    pub suppressed_since: Option<String>,
}

impl V1AlertExecution {
    pub fn new() -> V1AlertExecution {
        V1AlertExecution {
            adhoc: None,
            result: None,
            sent_notifications: None,
            execution_time: None,
            started_on: None,
            finished_on: None,
            suppressed_since: None,
        }
    }
}

