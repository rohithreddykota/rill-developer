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
pub struct V1NumericStatistics {
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "mean", skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    #[serde(rename = "q25", skip_serializing_if = "Option::is_none")]
    pub q25: Option<f64>,
    #[serde(rename = "q50", skip_serializing_if = "Option::is_none")]
    pub q50: Option<f64>,
    #[serde(rename = "q75", skip_serializing_if = "Option::is_none")]
    pub q75: Option<f64>,
    #[serde(rename = "sd", skip_serializing_if = "Option::is_none")]
    pub sd: Option<f64>,
}

impl V1NumericStatistics {
    pub fn new() -> V1NumericStatistics {
        V1NumericStatistics {
            min: None,
            max: None,
            mean: None,
            q25: None,
            q50: None,
            q75: None,
            sd: None,
        }
    }
}

