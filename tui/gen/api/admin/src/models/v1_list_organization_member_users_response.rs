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
pub struct V1ListOrganizationMemberUsersResponse {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<models::V1MemberUser>>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl V1ListOrganizationMemberUsersResponse {
    pub fn new() -> V1ListOrganizationMemberUsersResponse {
        V1ListOrganizationMemberUsersResponse {
            members: None,
            next_page_token: None,
        }
    }
}

