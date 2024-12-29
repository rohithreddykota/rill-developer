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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum V1ReconcileStatus {
    #[serde(rename = "RECONCILE_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "RECONCILE_STATUS_IDLE")]
    Idle,
    #[serde(rename = "RECONCILE_STATUS_PENDING")]
    Pending,
    #[serde(rename = "RECONCILE_STATUS_RUNNING")]
    Running,

}

impl std::fmt::Display for V1ReconcileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "RECONCILE_STATUS_UNSPECIFIED"),
            Self::Idle => write!(f, "RECONCILE_STATUS_IDLE"),
            Self::Pending => write!(f, "RECONCILE_STATUS_PENDING"),
            Self::Running => write!(f, "RECONCILE_STATUS_RUNNING"),
        }
    }
}

impl Default for V1ReconcileStatus {
    fn default() -> V1ReconcileStatus {
        Self::Unspecified
    }
}
