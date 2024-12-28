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
pub struct V1Condition {
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub op: Option<models::Runtimev1Operation>,
    #[serde(rename = "exprs", skip_serializing_if = "Option::is_none")]
    pub exprs: Option<Vec<models::V1Expression>>,
}

impl V1Condition {
    pub fn new() -> V1Condition {
        V1Condition {
            op: None,
            exprs: None,
        }
    }
}

