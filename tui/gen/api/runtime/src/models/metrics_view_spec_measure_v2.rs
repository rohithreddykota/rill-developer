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
pub struct MetricsViewSpecMeasureV2 {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::MetricsViewSpecMeasureType>,
    #[serde(rename = "window", skip_serializing_if = "Option::is_none")]
    pub window: Option<Box<models::MetricsViewSpecMeasureWindow>>,
    #[serde(rename = "perDimensions", skip_serializing_if = "Option::is_none")]
    pub per_dimensions: Option<Vec<models::MetricsViewSpecDimensionSelector>>,
    #[serde(rename = "requiredDimensions", skip_serializing_if = "Option::is_none")]
    pub required_dimensions: Option<Vec<models::MetricsViewSpecDimensionSelector>>,
    #[serde(rename = "referencedMeasures", skip_serializing_if = "Option::is_none")]
    pub referenced_measures: Option<Vec<String>>,
    #[serde(rename = "formatPreset", skip_serializing_if = "Option::is_none")]
    pub format_preset: Option<String>,
    #[serde(rename = "formatD3", skip_serializing_if = "Option::is_none")]
    pub format_d3: Option<String>,
    #[serde(rename = "formatD3Locale", skip_serializing_if = "Option::is_none")]
    pub format_d3_locale: Option<serde_json::Value>,
    #[serde(rename = "validPercentOfTotal", skip_serializing_if = "Option::is_none")]
    pub valid_percent_of_total: Option<bool>,
}

impl MetricsViewSpecMeasureV2 {
    pub fn new() -> MetricsViewSpecMeasureV2 {
        MetricsViewSpecMeasureV2 {
            name: None,
            display_name: None,
            description: None,
            expression: None,
            r#type: None,
            window: None,
            per_dimensions: None,
            required_dimensions: None,
            referenced_measures: None,
            format_preset: None,
            format_d3: None,
            format_d3_locale: None,
            valid_percent_of_total: None,
        }
    }
}
