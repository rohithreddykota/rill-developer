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

/// V1FileEvent : FileEvent describes a file change.
/// FileEvent describes a file change.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum V1FileEvent {
    #[serde(rename = "FILE_EVENT_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "FILE_EVENT_WRITE")]
    Write,
    #[serde(rename = "FILE_EVENT_DELETE")]
    Delete,

}

impl std::fmt::Display for V1FileEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "FILE_EVENT_UNSPECIFIED"),
            Self::Write => write!(f, "FILE_EVENT_WRITE"),
            Self::Delete => write!(f, "FILE_EVENT_DELETE"),
        }
    }
}

impl Default for V1FileEvent {
    fn default() -> V1FileEvent {
        Self::Unspecified
    }
}

