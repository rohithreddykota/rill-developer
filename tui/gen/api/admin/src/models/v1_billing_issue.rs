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
pub struct V1BillingIssue {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::V1BillingIssueType>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<models::V1BillingIssueLevel>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::V1BillingIssueMetadata>>,
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(rename = "createdOn", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
}

impl V1BillingIssue {
    pub fn new() -> V1BillingIssue {
        V1BillingIssue {
            organization: None,
            r#type: None,
            level: None,
            metadata: None,
            event_time: None,
            created_on: None,
        }
    }
}

