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
pub struct V1DirEntry {
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "isDir", skip_serializing_if = "Option::is_none")]
    pub is_dir: Option<bool>,
}

impl V1DirEntry {
    pub fn new() -> V1DirEntry {
        V1DirEntry {
            path: None,
            is_dir: None,
        }
    }
}

