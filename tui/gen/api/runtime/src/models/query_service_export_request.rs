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
pub struct QueryServiceExportRequest {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<models::V1ExportFormat>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<models::V1Query>>,
    #[serde(rename = "bakedQuery", skip_serializing_if = "Option::is_none")]
    pub baked_query: Option<String>,
}

impl QueryServiceExportRequest {
    pub fn new() -> QueryServiceExportRequest {
        QueryServiceExportRequest {
            limit: None,
            format: None,
            query: None,
            baked_query: None,
        }
    }
}

