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
pub struct V1AddOrganizationMemberUserResponse {
    #[serde(rename = "pendingSignup", skip_serializing_if = "Option::is_none")]
    pub pending_signup: Option<bool>,
}

impl V1AddOrganizationMemberUserResponse {
    pub fn new() -> V1AddOrganizationMemberUserResponse {
        V1AddOrganizationMemberUserResponse {
            pending_signup: None,
        }
    }
}
