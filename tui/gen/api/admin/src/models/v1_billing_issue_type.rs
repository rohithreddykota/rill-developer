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
pub enum V1BillingIssueType {
    #[serde(rename = "BILLING_ISSUE_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BILLING_ISSUE_TYPE_ON_TRIAL")]
    OnTrial,
    #[serde(rename = "BILLING_ISSUE_TYPE_TRIAL_ENDED")]
    TrialEnded,
    #[serde(rename = "BILLING_ISSUE_TYPE_NO_PAYMENT_METHOD")]
    NoPaymentMethod,
    #[serde(rename = "BILLING_ISSUE_TYPE_NO_BILLABLE_ADDRESS")]
    NoBillableAddress,
    #[serde(rename = "BILLING_ISSUE_TYPE_PAYMENT_FAILED")]
    PaymentFailed,
    #[serde(rename = "BILLING_ISSUE_TYPE_SUBSCRIPTION_CANCELLED")]
    SubscriptionCancelled,
    #[serde(rename = "BILLING_ISSUE_TYPE_NEVER_SUBSCRIBED")]
    NeverSubscribed,

}

impl std::fmt::Display for V1BillingIssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "BILLING_ISSUE_TYPE_UNSPECIFIED"),
            Self::OnTrial => write!(f, "BILLING_ISSUE_TYPE_ON_TRIAL"),
            Self::TrialEnded => write!(f, "BILLING_ISSUE_TYPE_TRIAL_ENDED"),
            Self::NoPaymentMethod => write!(f, "BILLING_ISSUE_TYPE_NO_PAYMENT_METHOD"),
            Self::NoBillableAddress => write!(f, "BILLING_ISSUE_TYPE_NO_BILLABLE_ADDRESS"),
            Self::PaymentFailed => write!(f, "BILLING_ISSUE_TYPE_PAYMENT_FAILED"),
            Self::SubscriptionCancelled => write!(f, "BILLING_ISSUE_TYPE_SUBSCRIPTION_CANCELLED"),
            Self::NeverSubscribed => write!(f, "BILLING_ISSUE_TYPE_NEVER_SUBSCRIBED"),
        }
    }
}

impl Default for V1BillingIssueType {
    fn default() -> V1BillingIssueType {
        Self::Unspecified
    }
}

