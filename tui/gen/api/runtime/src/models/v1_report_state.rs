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
pub struct V1ReportState {
    #[serde(rename = "nextRunOn", skip_serializing_if = "Option::is_none")]
    pub next_run_on: Option<String>,
    #[serde(rename = "currentExecution", skip_serializing_if = "Option::is_none")]
    pub current_execution: Option<Box<models::V1ReportExecution>>,
    #[serde(rename = "executionHistory", skip_serializing_if = "Option::is_none")]
    pub execution_history: Option<Vec<models::V1ReportExecution>>,
    #[serde(rename = "executionCount", skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<i64>,
}

impl V1ReportState {
    pub fn new() -> V1ReportState {
        V1ReportState {
            next_run_on: None,
            current_execution: None,
            execution_history: None,
            execution_count: None,
        }
    }
}

