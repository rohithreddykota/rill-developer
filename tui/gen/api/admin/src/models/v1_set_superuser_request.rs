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
pub struct V1SetSuperuserRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "superuser", skip_serializing_if = "Option::is_none")]
    pub superuser: Option<bool>,
}

impl V1SetSuperuserRequest {
    pub fn new() -> V1SetSuperuserRequest {
        V1SetSuperuserRequest {
            email: None,
            superuser: None,
        }
    }
}
