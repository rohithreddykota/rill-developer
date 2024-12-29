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
pub struct V1ReportExecution {
    #[serde(rename = "adhoc", skip_serializing_if = "Option::is_none")]
    pub adhoc: Option<bool>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "reportTime", skip_serializing_if = "Option::is_none")]
    pub report_time: Option<String>,
    #[serde(rename = "startedOn", skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    #[serde(rename = "finishedOn", skip_serializing_if = "Option::is_none")]
    pub finished_on: Option<String>,
}

impl V1ReportExecution {
    pub fn new() -> V1ReportExecution {
        V1ReportExecution {
            adhoc: None,
            error_message: None,
            report_time: None,
            started_on: None,
            finished_on: None,
        }
    }
}
