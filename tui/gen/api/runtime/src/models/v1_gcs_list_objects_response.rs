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
pub struct V1GcsListObjectsResponse {
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "objects", skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<models::V1GcsObject>>,
}

impl V1GcsListObjectsResponse {
    pub fn new() -> V1GcsListObjectsResponse {
        V1GcsListObjectsResponse {
            next_page_token: None,
            objects: None,
        }
    }
}

