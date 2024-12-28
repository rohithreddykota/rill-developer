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
pub struct V1MetricsViewSpec {
    #[serde(rename = "connector", skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "databaseSchema", skip_serializing_if = "Option::is_none")]
    pub database_schema: Option<String>,
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// Name of the model the metrics view is based on. Either table or model should be set.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "timeDimension", skip_serializing_if = "Option::is_none")]
    pub time_dimension: Option<String>,
    #[serde(rename = "smallestTimeGrain", skip_serializing_if = "Option::is_none")]
    pub smallest_time_grain: Option<models::V1TimeGrain>,
    /// Expression to evaluate a watermark for the metrics view. If not set, the watermark defaults to max(time_dimension).
    #[serde(rename = "watermarkExpression", skip_serializing_if = "Option::is_none")]
    pub watermark_expression: Option<String>,
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<models::MetricsViewSpecDimensionV2>>,
    #[serde(rename = "measures", skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<models::MetricsViewSpecMeasureV2>>,
    #[serde(rename = "securityRules", skip_serializing_if = "Option::is_none")]
    pub security_rules: Option<Vec<models::V1SecurityRule>>,
    /// ISO 8601 weekday number to use as the base for time aggregations by week. Defaults to 1 (Monday).
    #[serde(rename = "firstDayOfWeek", skip_serializing_if = "Option::is_none")]
    pub first_day_of_week: Option<i64>,
    /// Month number to use as the base for time aggregations by year. Defaults to 1 (January).
    #[serde(rename = "firstMonthOfYear", skip_serializing_if = "Option::is_none")]
    pub first_month_of_year: Option<i64>,
    /// List of selected dimensions by defaults. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "defaultDimensions", skip_serializing_if = "Option::is_none")]
    pub default_dimensions: Option<Vec<String>>,
    /// List of selected measures by defaults. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "defaultMeasures", skip_serializing_if = "Option::is_none")]
    pub default_measures: Option<Vec<String>>,
    /// Default time range for the dashboard. It should be a valid ISO 8601 duration string. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "defaultTimeRange", skip_serializing_if = "Option::is_none")]
    pub default_time_range: Option<String>,
    #[serde(rename = "defaultComparisonMode", skip_serializing_if = "Option::is_none")]
    pub default_comparison_mode: Option<models::MetricsViewSpecComparisonMode>,
    /// If comparison mode is dimension then this determines which is the default dimension. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "defaultComparisonDimension", skip_serializing_if = "Option::is_none")]
    pub default_comparison_dimension: Option<String>,
    /// Default theme to apply. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "defaultTheme", skip_serializing_if = "Option::is_none")]
    pub default_theme: Option<String>,
    /// List of available time ranges with comparison ranges that would replace the default list. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "availableTimeRanges", skip_serializing_if = "Option::is_none")]
    pub available_time_ranges: Option<Vec<models::MetricsViewSpecAvailableTimeRange>>,
    /// Available time zones list preferred time zones using IANA location identifiers. Deprecated: Now defined in the Explore resource.
    #[serde(rename = "availableTimeZones", skip_serializing_if = "Option::is_none")]
    pub available_time_zones: Option<Vec<String>>,
}

impl V1MetricsViewSpec {
    pub fn new() -> V1MetricsViewSpec {
        V1MetricsViewSpec {
            connector: None,
            database: None,
            database_schema: None,
            table: None,
            model: None,
            display_name: None,
            description: None,
            time_dimension: None,
            smallest_time_grain: None,
            watermark_expression: None,
            dimensions: None,
            measures: None,
            security_rules: None,
            first_day_of_week: None,
            first_month_of_year: None,
            default_dimensions: None,
            default_measures: None,
            default_time_range: None,
            default_comparison_mode: None,
            default_comparison_dimension: None,
            default_theme: None,
            available_time_ranges: None,
            available_time_zones: None,
        }
    }
}

