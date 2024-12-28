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
pub struct AdminServiceIssueMagicAuthTokenRequest {
    /// TTL for the token in minutes. Set to 0 for no expiry. Defaults to no expiry.
    #[serde(rename = "ttlMinutes", skip_serializing_if = "Option::is_none")]
    pub ttl_minutes: Option<String>,
    /// Type of resource to grant access to. Currently only supports \"rill.runtime.v1.Explore\".
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// Name of the resource to grant access to.
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<models::V1Expression>>,
    /// Optional list of fields to limit access to. If empty, no field access rule will be added. This will be translated to a rill.runtime.v1.SecurityRuleFieldAccess, which currently applies to dimension and measure names for explores and metrics views.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Optional state to store with the token. Can be fetched with GetCurrentMagicAuthToken.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Optional display name to store with the token.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl AdminServiceIssueMagicAuthTokenRequest {
    pub fn new() -> AdminServiceIssueMagicAuthTokenRequest {
        AdminServiceIssueMagicAuthTokenRequest {
            ttl_minutes: None,
            resource_type: None,
            resource_name: None,
            filter: None,
            fields: None,
            state: None,
            display_name: None,
        }
    }
}

