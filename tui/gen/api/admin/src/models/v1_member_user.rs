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
pub struct V1MemberUser {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userEmail", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userPhotoUrl", skip_serializing_if = "Option::is_none")]
    pub user_photo_url: Option<String>,
    #[serde(rename = "roleName", skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "createdOn", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "updatedOn", skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,
}

impl V1MemberUser {
    pub fn new() -> V1MemberUser {
        V1MemberUser {
            user_id: None,
            user_email: None,
            user_name: None,
            user_photo_url: None,
            role_name: None,
            created_on: None,
            updated_on: None,
        }
    }
}

