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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1GetCloneCredentialsResponse {
    #[serde(rename = "gitRepoUrl", skip_serializing_if = "Option::is_none")]
    pub git_repo_url: Option<String>,
    #[serde(rename = "gitUsername", skip_serializing_if = "Option::is_none")]
    pub git_username: Option<String>,
    #[serde(rename = "gitPassword", skip_serializing_if = "Option::is_none")]
    pub git_password: Option<String>,
    #[serde(rename = "gitSubpath", skip_serializing_if = "Option::is_none")]
    pub git_subpath: Option<String>,
    #[serde(rename = "gitProdBranch", skip_serializing_if = "Option::is_none")]
    pub git_prod_branch: Option<String>,
    #[serde(rename = "archiveDownloadUrl", skip_serializing_if = "Option::is_none")]
    pub archive_download_url: Option<String>,
}

impl V1GetCloneCredentialsResponse {
    pub fn new() -> V1GetCloneCredentialsResponse {
        V1GetCloneCredentialsResponse {
            git_repo_url: None,
            git_username: None,
            git_password: None,
            git_subpath: None,
            git_prod_branch: None,
            archive_download_url: None,
        }
    }
}

