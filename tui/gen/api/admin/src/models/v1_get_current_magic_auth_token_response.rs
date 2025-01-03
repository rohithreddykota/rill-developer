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
pub struct V1GetCurrentMagicAuthTokenResponse {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<Box<models::V1MagicAuthToken>>,
}

impl V1GetCurrentMagicAuthTokenResponse {
    pub fn new() -> V1GetCurrentMagicAuthTokenResponse {
        V1GetCurrentMagicAuthTokenResponse {
            token: None,
        }
    }
}

