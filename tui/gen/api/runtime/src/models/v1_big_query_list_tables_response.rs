/*
 * rill/runtime/v1/api.proto
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
pub struct V1BigQueryListTablesResponse {
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

impl V1BigQueryListTablesResponse {
    pub fn new() -> V1BigQueryListTablesResponse {
        V1BigQueryListTablesResponse {
            next_page_token: None,
            names: None,
        }
    }
}
