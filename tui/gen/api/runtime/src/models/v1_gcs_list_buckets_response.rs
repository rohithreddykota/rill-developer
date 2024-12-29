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
pub struct V1GcsListBucketsResponse {
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "buckets", skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<String>>,
}

impl V1GcsListBucketsResponse {
    pub fn new() -> V1GcsListBucketsResponse {
        V1GcsListBucketsResponse {
            next_page_token: None,
            buckets: None,
        }
    }
}
