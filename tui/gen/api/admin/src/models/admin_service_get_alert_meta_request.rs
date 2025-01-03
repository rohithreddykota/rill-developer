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
pub struct AdminServiceGetAlertMetaRequest {
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "queryForUserId", skip_serializing_if = "Option::is_none")]
    pub query_for_user_id: Option<String>,
    #[serde(rename = "queryForUserEmail", skip_serializing_if = "Option::is_none")]
    pub query_for_user_email: Option<String>,
}

impl AdminServiceGetAlertMetaRequest {
    pub fn new() -> AdminServiceGetAlertMetaRequest {
        AdminServiceGetAlertMetaRequest {
            branch: None,
            alert: None,
            annotations: None,
            query_for_user_id: None,
            query_for_user_email: None,
        }
    }
}

