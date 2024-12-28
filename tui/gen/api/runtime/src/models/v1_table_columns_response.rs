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
pub struct V1TableColumnsResponse {
    #[serde(rename = "profileColumns", skip_serializing_if = "Option::is_none")]
    pub profile_columns: Option<Vec<models::V1ProfileColumn>>,
    #[serde(rename = "unsupportedColumns", skip_serializing_if = "Option::is_none")]
    pub unsupported_columns: Option<std::collections::HashMap<String, String>>,
}

impl V1TableColumnsResponse {
    pub fn new() -> V1TableColumnsResponse {
        V1TableColumnsResponse {
            profile_columns: None,
            unsupported_columns: None,
        }
    }
}

