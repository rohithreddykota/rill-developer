/*
 * rill/admin/v1/ai.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: version not set
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum V1DeploymentStatus {
    #[serde(rename = "DEPLOYMENT_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "DEPLOYMENT_STATUS_PENDING")]
    Pending,
    #[serde(rename = "DEPLOYMENT_STATUS_OK")]
    Ok,
    #[serde(rename = "DEPLOYMENT_STATUS_ERROR")]
    Error,

}

impl std::fmt::Display for V1DeploymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "DEPLOYMENT_STATUS_UNSPECIFIED"),
            Self::Pending => write!(f, "DEPLOYMENT_STATUS_PENDING"),
            Self::Ok => write!(f, "DEPLOYMENT_STATUS_OK"),
            Self::Error => write!(f, "DEPLOYMENT_STATUS_ERROR"),
        }
    }
}

impl Default for V1DeploymentStatus {
    fn default() -> V1DeploymentStatus {
        Self::Unspecified
    }
}
