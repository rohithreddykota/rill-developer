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
pub struct V1OlapListTablesResponse {
    #[serde(rename = "tables", skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<models::V1TableInfo>>,
}

impl V1OlapListTablesResponse {
    pub fn new() -> V1OlapListTablesResponse {
        V1OlapListTablesResponse {
            tables: None,
        }
    }
}

