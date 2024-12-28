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
pub struct V1Subquery {
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(rename = "measures", skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<String>>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<models::V1Expression>>,
    #[serde(rename = "having", skip_serializing_if = "Option::is_none")]
    pub having: Option<Box<models::V1Expression>>,
}

impl V1Subquery {
    pub fn new() -> V1Subquery {
        V1Subquery {
            dimension: None,
            measures: None,
            r#where: None,
            having: None,
        }
    }
}

