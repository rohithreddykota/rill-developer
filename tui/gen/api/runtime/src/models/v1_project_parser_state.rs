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
pub struct V1ProjectParserState {
    #[serde(rename = "parseErrors", skip_serializing_if = "Option::is_none")]
    pub parse_errors: Option<Vec<models::V1ParseError>>,
    #[serde(rename = "currentCommitSha", skip_serializing_if = "Option::is_none")]
    pub current_commit_sha: Option<String>,
    #[serde(rename = "watching", skip_serializing_if = "Option::is_none")]
    pub watching: Option<bool>,
}

impl V1ProjectParserState {
    pub fn new() -> V1ProjectParserState {
        V1ProjectParserState {
            parse_errors: None,
            current_commit_sha: None,
            watching: None,
        }
    }
}
