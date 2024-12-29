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
pub enum V1GithubPermission {
    #[serde(rename = "GITHUB_PERMISSION_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "GITHUB_PERMISSION_READ")]
    Read,
    #[serde(rename = "GITHUB_PERMISSION_WRITE")]
    Write,

}

impl std::fmt::Display for V1GithubPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "GITHUB_PERMISSION_UNSPECIFIED"),
            Self::Read => write!(f, "GITHUB_PERMISSION_READ"),
            Self::Write => write!(f, "GITHUB_PERMISSION_WRITE"),
        }
    }
}

impl Default for V1GithubPermission {
    fn default() -> V1GithubPermission {
        Self::Unspecified
    }
}
