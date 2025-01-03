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
pub struct V1ListProjectWhitelistedDomainsResponse {
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<models::V1WhitelistedDomain>>,
}

impl V1ListProjectWhitelistedDomainsResponse {
    pub fn new() -> V1ListProjectWhitelistedDomainsResponse {
        V1ListProjectWhitelistedDomainsResponse {
            domains: None,
        }
    }
}

