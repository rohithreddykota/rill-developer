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
pub struct V1GetPaymentsPortalUrlResponse {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl V1GetPaymentsPortalUrlResponse {
    pub fn new() -> V1GetPaymentsPortalUrlResponse {
        V1GetPaymentsPortalUrlResponse {
            url: None,
        }
    }
}
