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
pub struct V1SudoUpdateUserQuotasRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "singleuserOrgs", skip_serializing_if = "Option::is_none")]
    pub singleuser_orgs: Option<i32>,
    #[serde(rename = "trialOrgs", skip_serializing_if = "Option::is_none")]
    pub trial_orgs: Option<i32>,
}

impl V1SudoUpdateUserQuotasRequest {
    pub fn new() -> V1SudoUpdateUserQuotasRequest {
        V1SudoUpdateUserQuotasRequest {
            email: None,
            singleuser_orgs: None,
            trial_orgs: None,
        }
    }
}

