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
pub struct V1ConnectorSpec {
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templatedProperties", skip_serializing_if = "Option::is_none")]
    pub templated_properties: Option<Vec<String>>,
    #[serde(rename = "provision", skip_serializing_if = "Option::is_none")]
    pub provision: Option<bool>,
    #[serde(rename = "provisionArgs", skip_serializing_if = "Option::is_none")]
    pub provision_args: Option<serde_json::Value>,
    /// DEPRECATED: properties_from_variables stores properties whose value is a variable. NOTE : properties_from_variables and properties both should be used to get all properties.
    #[serde(rename = "propertiesFromVariables", skip_serializing_if = "Option::is_none")]
    pub properties_from_variables: Option<std::collections::HashMap<String, String>>,
}

impl V1ConnectorSpec {
    pub fn new() -> V1ConnectorSpec {
        V1ConnectorSpec {
            driver: None,
            properties: None,
            templated_properties: None,
            provision: None,
            provision_args: None,
            properties_from_variables: None,
        }
    }
}

