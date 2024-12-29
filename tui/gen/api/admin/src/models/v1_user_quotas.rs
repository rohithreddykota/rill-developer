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
pub struct V1UserQuotas {
    #[serde(rename = "singleuserOrgs", skip_serializing_if = "Option::is_none")]
    pub singleuser_orgs: Option<i32>,
    #[serde(rename = "trialOrgs", skip_serializing_if = "Option::is_none")]
    pub trial_orgs: Option<i32>,
}

impl V1UserQuotas {
    pub fn new() -> V1UserQuotas {
        V1UserQuotas {
            singleuser_orgs: None,
            trial_orgs: None,
        }
    }
}
